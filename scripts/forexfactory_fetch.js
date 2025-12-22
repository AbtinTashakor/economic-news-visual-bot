const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

(async () => {
    const outDir = path.join(__dirname, '..', 'tmp');
    const outFile = path.join(outDir, 'forexfactory.json');

    if (!fs.existsSync(outDir)) fs.mkdirSync(outDir, { recursive: true });

    const browser = await chromium.launch({ headless: true });

    const context = await browser.newContext({
        locale: 'en-US',
        timezoneId: 'America/New_York',
        userAgent:
            'Mozilla/5.0 (Windows NT 10.0; Win64; x64) ' +
            'AppleWebKit/537.36 (KHTML, like Gecko) ' +
            'Chrome/120.0.0.0 Safari/537.36',
    });

    const page = await context.newPage();

    await page.goto('https://www.forexfactory.com/calendar?day=today', {
        waitUntil: 'networkidle',
        timeout: 60000
    });

    // صبر کن تا calendar آماده شود
    await page.waitForTimeout(3000);

    const calendarState = await page.evaluate(() => {
        const states = window.calendarComponentStates;
        const firstKey = Object.keys(states)[0];
        return states[firstKey];
    });

    // JS → JSON واقعی
    fs.writeFileSync(outFile, JSON.stringify(calendarState), 'utf8');

    await browser.close();

    console.log(`Saved JSON to: ${outFile}`);
})();
