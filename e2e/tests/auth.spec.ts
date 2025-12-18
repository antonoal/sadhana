import { test, expect } from '@playwright/test';

test('user can register and login', async ({ page }) => {
  await page.goto('/');

  await page.getByRole('link', { name: 'Register' }).click();
  await page.getByLabel('Email').fill('test@example.com');
  await page.getByLabel('Password').fill('password123');
  await page.getByRole('button', { name: 'Register' }).click();

  await expect(page.getByText('Welcome')).toBeVisible();
});