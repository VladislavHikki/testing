// tests/e2e/login.spec.js
import { test, expect } from '@playwright/test';

const url = 'https://client2.test.itroom18.ru';

test.describe('Login Form', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(`${url}/auth`);
  });

  test('should display validation errors for empty fields', async ({ page }) => {
    await page.click('button[type=submit]');
    expect((await page.locator('.error').all()).length).toBe(2);
  });

  test('should display validation error for invalid email', async ({ page }) => {
    await page.fill('#email', 'invalid-email');
    await page.click('button[type=submit]');
    expect((await page.locator('.error').all()).at()).toContain('Некорректная почта');
  });

  test('should handle incorrect login credentials', async ({ page }) => {
    await page.fill('#email', 'user@example.com');
    await page.fill('input[type=password]', 'wrongpassword');
    await page.click('button[type=submit]');

    await expect(page.locator('.error')).toHaveText('Неправильный логин или пароль');
  });

  test('should navigate to profile page on successful login', async ({ page }) => {
    await page.fill('#email', 'vlad19022005@mail.ru');
    await page.fill('input[type=password]', '1');

    page.route(`${url}/api/users/auth`, (route) =>
      route.fulfill({
        status: 200,
        body: JSON.stringify({ email: 'vlad19022005@mail.ru', password: '1', rememberMe: false }),
      })
    );

    await page.click('button[type=submit]');
    await expect(page).toHaveURL(`${url}/profile/872`);
  });

  test('should toggle "remember me" switch', async ({ page }) => {
    const switchLocator = page.locator('.p-inputswitch-input');
    await expect(switchLocator).not.toBeChecked();
    await switchLocator.click();
    await expect(switchLocator).toBeChecked();
  });
});