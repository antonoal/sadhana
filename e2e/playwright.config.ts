import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',

  /* Fail fast in CI */
  forbidOnly: !!process.env.CI,

  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,

  /* Parallelism */
  workers: process.env.CI ? 2 : undefined,

  /* Test output */
  reporter: [
    ['html', { open: 'never' }]
  ],

  /* Shared settings */
  use: {
    baseURL: 'http://localhost:3000',

    headless: true,

    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',

    viewport: { width: 1280, height: 800 }
  },

  /* Browsers */
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] }
    },
    // Uncomment later if needed:
    // {
    //   name: 'firefox',
    //   use: { ...devices['Desktop Firefox'] }
    // },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] }
    }
  ]
});
