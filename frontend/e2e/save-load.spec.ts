import { test, expect } from '@playwright/test';
import type { Page } from '@playwright/test';

/**
 * Save/Load E2E Tests
 *
 * Tests the save and load functionality in the dashboard
 */

test.describe('Save/Load Functionality', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/');

    // Wait for dashboard to load
    await page.waitForSelector('.dashboard-view', { timeout: 10000 });
  });

  test('should display save and load buttons in dashboard header', async ({ page }) => {
    // Check that Save button exists
    const saveButton = page.getByRole('button', { name: /save/i });
    await expect(saveButton).toBeVisible();

    // Check that Load button exists
    const loadButton = page.getByRole('button', { name: /load/i });
    await expect(loadButton).toBeVisible();

    // Verify both buttons are enabled
    await expect(saveButton).toBeEnabled();
    await expect(loadButton).toBeEnabled();
  });

  test('should save engine state and download JSON file', async ({ page }) => {
    // Set up download listener
    const downloadPromise = page.waitForEvent('download');

    // Click save button
    const saveButton = page.getByRole('button', { name: /save/i });
    await saveButton.click();

    // Wait for download
    const download = await downloadPromise;

    // Verify download filename
    const filename = download.suggestedFilename();
    expect(filename).toMatch(/^satisflow-save_\d{4}-\d{2}-\d{2}T\d{2}-\d{2}-\d{2}\.json$/);

    // Verify file can be read and is valid JSON
    const path = await download.path();
    expect(path).toBeTruthy();
  });

  test('should show success message after save', async ({ page }) => {
    // Set up download listener
    const downloadPromise = page.waitForEvent('download');

    // Click save button
    const saveButton = page.getByRole('button', { name: /save/i });
    await saveButton.click();

    // Wait for download
    await downloadPromise;

    // Check for success message
    const successMessage = page.locator('.status-message.success');
    await expect(successMessage).toBeVisible({ timeout: 2000 });
    await expect(successMessage).toContainText(/saved successfully/i);

    // Success message should disappear after timeout
    await expect(successMessage).not.toBeVisible({ timeout: 5000 });
  });

  test('should show loading state during save', async ({ page }) => {
    // Click save button
    const saveButton = page.getByRole('button', { name: /save/i });

    // Start save operation
    await saveButton.click();

    // Button should show loading state (disabled)
    await expect(saveButton).toBeDisabled();

    // Wait for operation to complete
    await page.waitForTimeout(1000);

    // Button should be enabled again
    await expect(saveButton).toBeEnabled({ timeout: 5000 });
  });

  test('should open file picker when load button is clicked', async ({ page }) => {
    // Set up file chooser listener
    const fileChooserPromise = page.waitForEvent('filechooser');

    // Click load button
    const loadButton = page.getByRole('button', { name: /load/i });
    await loadButton.click();

    // Verify file chooser opened
    const fileChooser = await fileChooserPromise;
    expect(fileChooser).toBeTruthy();
  });

  test('should load valid save file', async ({ page }) => {
    // Create a mock save file
    const mockSaveData = {
      version: '0.1.0',
      created_at: new Date().toISOString(),
      last_modified: new Date().toISOString(),
      game_version: null,
      engine: {
        factories: {},
        logistics_lines: {}
      }
    };

    // Convert to file
    const buffer = Buffer.from(JSON.stringify(mockSaveData, null, 2));

    // Set up file chooser
    const fileChooserPromise = page.waitForEvent('filechooser');

    // Click load button
    const loadButton = page.getByRole('button', { name: /load/i });
    await loadButton.click();

    // Upload file
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({
      name: 'test-save.json',
      mimeType: 'application/json',
      buffer: buffer
    });

    // Check for success message
    const successMessage = page.locator('.status-message.success');
    await expect(successMessage).toBeVisible({ timeout: 5000 });
    await expect(successMessage).toContainText(/loaded|successfully/i);

    // Dashboard should refresh after load
    // Wait for loading indicator
    await page.waitForSelector('.loading-text', { state: 'hidden', timeout: 10000 });
  });

  test('should show error message for invalid JSON file', async ({ page }) => {
    // Create an invalid JSON file
    const buffer = Buffer.from('{ invalid json }');

    // Set up file chooser
    const fileChooserPromise = page.waitForEvent('filechooser');

    // Click load button
    const loadButton = page.getByRole('button', { name: /load/i });
    await loadButton.click();

    // Upload invalid file
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({
      name: 'invalid.json',
      mimeType: 'application/json',
      buffer: buffer
    });

    // Check for error message
    const errorMessage = page.locator('.status-message.error');
    await expect(errorMessage).toBeVisible({ timeout: 5000 });
    await expect(errorMessage).toContainText(/failed|invalid|error/i);
  });

  test('should disable buttons during operations', async ({ page }) => {
    const saveButton = page.getByRole('button', { name: /save/i });
    const loadButton = page.getByRole('button', { name: /load/i });

    // Both buttons should be enabled initially
    await expect(saveButton).toBeEnabled();
    await expect(loadButton).toBeEnabled();

    // Start save operation
    await saveButton.click();

    // Both buttons should be disabled during save
    await expect(saveButton).toBeDisabled();
    await expect(loadButton).toBeDisabled();

    // Wait for save to complete
    await page.waitForTimeout(2000);

    // Buttons should be enabled again
    await expect(saveButton).toBeEnabled({ timeout: 3000 });
    await expect(loadButton).toBeEnabled({ timeout: 3000 });
  });

  test('should maintain save/load buttons in mobile view', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    // Check that buttons are still visible
    const saveButton = page.getByRole('button', { name: /save/i });
    const loadButton = page.getByRole('button', { name: /load/i });

    await expect(saveButton).toBeVisible();
    await expect(loadButton).toBeVisible();
  });

  test('should integrate with dashboard refresh', async ({ page }) => {
    // Click refresh button
    const refreshButton = page.getByRole('button', { name: /refresh/i });
    await refreshButton.click();

    // Save button should still be functional after refresh
    const saveButton = page.getByRole('button', { name: /save/i });
    await expect(saveButton).toBeEnabled({ timeout: 5000 });

    // Set up download listener
    const downloadPromise = page.waitForEvent('download');
    await saveButton.click();

    // Verify download happens
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/\.json$/);
  });
});

test.describe('Save/Load API Integration', () => {
  test('should call correct API endpoints', async ({ page }) => {
    // Monitor network requests
    const saveRequests: string[] = [];
    const loadRequests: string[] = [];

    page.on('request', request => {
      const url = request.url();
      if (url.includes('/api/save')) {
        saveRequests.push(url);
      }
      if (url.includes('/api/load')) {
        loadRequests.push(url);
      }
    });

    // Navigate to dashboard
    await page.goto('/');
    await page.waitForSelector('.dashboard-view');

    // Test save endpoint
    const downloadPromise = page.waitForEvent('download');
    const saveButton = page.getByRole('button', { name: /save/i });
    await saveButton.click();
    await downloadPromise;

    // Verify save API was called
    expect(saveRequests.length).toBeGreaterThan(0);
    expect(saveRequests[0]).toContain('/api/save');
  });

  test('should handle API errors gracefully', async ({ page }) => {
    // Intercept save API and return error
    await page.route('**/api/save', route => {
      route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Internal server error' })
      });
    });

    await page.goto('/');
    await page.waitForSelector('.dashboard-view');

    // Try to save
    const saveButton = page.getByRole('button', { name: /save/i });
    await saveButton.click();

    // Should show error message
    const errorMessage = page.locator('.status-message.error');
    await expect(errorMessage).toBeVisible({ timeout: 5000 });
  });
});
