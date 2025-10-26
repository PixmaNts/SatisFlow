import { test, expect } from '@playwright/test'

test.describe('Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard page
    await page.goto('/')
  })

  test('displays dashboard title', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Dashboard')
  })

  test('displays summary statistics', async ({ page }) => {
    // Wait for data to load
    await page.waitForSelector('.dashboard-stats')

    // Check that summary cards are displayed
    await expect(page.locator('.stat-card')).toHaveCount(5)

    // Check that specific stats are displayed
    await expect(page.locator('.factories-stat')).toBeVisible()
    await expect(page.locator('.production-lines-stat')).toBeVisible()
    await expect(page.locator('.logistics-lines-stat')).toBeVisible()
    await expect(page.locator('.power-consumption-stat')).toBeVisible()
    await expect(page.locator('.power-generation-stat')).toBeVisible()
  })

  test('displays item balance table', async ({ page }) => {
    // Wait for data to load
    await page.waitForSelector('.item-balances')

    // Check that the table is displayed
    await expect(page.locator('.item-balances-table')).toBeVisible()

    // Check that table headers are displayed
    await expect(page.locator('th:has-text("Item")')).toBeVisible()
    await expect(page.locator('th:has-text("Balance")')).toBeVisible()
    await expect(page.locator('th:has-text("State")')).toBeVisible()
  })

  test('displays power statistics', async ({ page }) => {
    // Wait for data to load
    await page.waitForSelector('.power-stats')

    // Check that power stats are displayed
    await expect(page.locator('.total-generation')).toBeVisible()
    await expect(page.locator('.total-consumption')).toBeVisible()
    await expect(page.locator('.net-power')).toBeVisible()
  })

  test('can refresh dashboard data', async ({ page }) => {
    // Wait for initial data to load
    await page.waitForSelector('.dashboard-stats')

    // Click refresh button
    await page.click('.refresh-button')

    // Check that loading indicator appears
    await expect(page.locator('.loading-indicator')).toBeVisible()

    // Wait for data to reload
    await page.waitForSelector('.dashboard-stats')

    // Check that loading indicator is gone
    await expect(page.locator('.loading-indicator')).not.toBeVisible()
  })

  test('displays power status indicator correctly', async ({ page }) => {
    // Wait for data to load
    await page.waitForSelector('.power-status')

    // Check that power status is displayed
    await expect(page.locator('.power-status')).toBeVisible()

    // The power status could be balanced, surplus, or deficit
    // We just check that one of the status indicators is present
    const hasPowerStatus = await Promise.any([
      page.locator('.power-balanced').isVisible().catch(() => false),
      page.locator('.power-surplus').isVisible().catch(() => false),
      page.locator('.power-deficit').isVisible().catch(() => false)
    ])

    expect(hasPowerStatus).toBe(true)
  })

  test('can filter item balances by state', async ({ page }) => {
    // Wait for data to load
    await page.waitForSelector('.item-balances')

    // Click on overflow filter
    await page.click('.filter-overflow')

    // Check that only overflow items are displayed
    const rows = page.locator('.item-balances-table tbody tr')
    const count = await rows.count()

    if (count > 0) {
      // Check that all visible rows have overflow state
      for (let i = 0; i < count; i++) {
        await expect(rows.nth(i).locator('.state-overflow')).toBeVisible()
      }
    }
  })

  test('can search for specific items', async ({ page }) => {
    // Wait for data to load
    await page.waitForSelector('.item-balances')

    // Type in search box
    await page.fill('.item-search', 'Iron')

    // Wait for search results
    await page.waitForTimeout(500)

    // Check that search results are displayed
    const rows = page.locator('.item-balances-table tbody tr')
    const count = await rows.count()

    if (count > 0) {
      // Check that at least one row contains "Iron"
      let hasIronItem = false
      for (let i = 0; i < count; i++) {
        const text = await rows.nth(i).textContent()
        if (text && text.includes('Iron')) {
          hasIronItem = true
          break
        }
      }
      expect(hasIronItem).toBe(true)
    }
  })

  test('handles navigation to factory details', async ({ page }) => {
    // Wait for data to load
    await page.waitForSelector('.dashboard-stats')

    // Click on factories stat card
    await page.click('.factories-stat')

    // Check that navigation to factories page occurred
    await expect(page).toHaveURL('/factories')
  })

  test('handles navigation to logistics', async ({ page }) => {
    // Wait for data to load
    await page.waitForSelector('.dashboard-stats')

    // Click on logistics stat card
    await page.click('.logistics-lines-stat')

    // Check that navigation to logistics page occurred
    await expect(page).toHaveURL('/logistics')
  })

  test('displays error message when API fails', async ({ page }) => {
    // Mock API failure
    await page.route('/api/dashboard/summary', route => {
      route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Internal server error' })
      })
    })

    await page.reload()

    // Check that error message is displayed
    await expect(page.locator('.error-message')).toBeVisible()
    await expect(page.locator('.error-message')).toContainText('Failed to load dashboard data')
  })
})
