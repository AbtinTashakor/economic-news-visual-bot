const { chromium } = require('playwright');

const fs = require('fs');
const path = require('path');

// ---- read date argument ----
const dateArg = process.argv[2] || 'today';

const url = `https://www.forexfactory.com/calendar?day=${dateArg}`;

(async () => {
    const outDir = path.join(__dirname, '..', 'tmp');
    const outFile = path.join(outDir, 'forexfactory.json');

    if (!fs.existsSync(outDir)) fs.mkdirSync(outDir, { recursive: true });

    const browser = await chromium.launch({
        headless: true,
        args: [
            '--disable-blink-features=AutomationControlled',
            '--no-sandbox',
            '--disable-dev-shm-usage'
        ]
    });


    const context = await browser.newContext({
        userAgent:
            'Mozilla/5.0 (Windows NT 10.0; Win64; x64) ' +
            'AppleWebKit/537.36 (KHTML, like Gecko) ' +
            'Chrome/120.0.0.0 Safari/537.36',
        viewport: { width: 1366, height: 768 },
        locale: 'en-US',
        timezoneId: 'America/New_York',
    });

    await context.addInitScript(() => {
        Object.defineProperty(navigator, 'webdriver', {
            get: () => false,
        });
    });


    const page = await context.newPage();

    await page.goto(url, {
        waitUntil: 'domcontentloaded',
        timeout: 60000
    });

    // کمی مکث
    await page.waitForTimeout(1500);

    // اسکرول فیک
    await page.mouse.wheel(0, 800);
    await page.waitForTimeout(1000);

    const calendarState = await page.evaluate(() => {
        const states = window.calendarComponentStates;
        const firstKey = Object.keys(states)[0];
        return states[firstKey];
    });

    fs.writeFileSync(outFile, JSON.stringify(calendarState), 'utf8');

    await browser.close();

    console.log(`Saved JSON for ${dateArg} → ${outFile}`);
})();
