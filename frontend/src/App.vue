<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { useTheme } from '@/composables/useTheme'
import { registerCommonShortcuts } from '@/composables/useKeyboardShortcuts'
import { ErrorBoundary, ToastContainer } from '@/components/ui'
import { useDashboardStore } from '@/stores'

// Initialize industrial dark theme
useTheme()

// Dashboard store for badge counts
const dashboardStore = useDashboardStore()

// Computed properties for badges
const totalFactories = computed(() => dashboardStore.totalFactories)
const totalLogisticsLines = computed(() => dashboardStore.totalLogisticsLines)

// Set up keyboard shortcuts (only if in browser environment)
if (typeof window !== 'undefined' && window.document) {
  onMounted(() => {
    const unregisterShortcuts = registerCommonShortcuts()

    // Fetch summary data for navbar badges
    dashboardStore.fetchSummary().catch(err => {
      console.error('Failed to fetch dashboard summary for navbar:', err)
    })

    // Clean up on unmount
    onUnmounted(() => {
      unregisterShortcuts()
    })
  })
}
</script>

<template>
  <div id="app" class="dark-theme">
    <ErrorBoundary fallback="default">
      <nav class="main-nav">
        <h1 class="nav-title">Satisflow</h1>
        <div class="nav-links">
          <router-link to="/" class="nav-link">Dashboard</router-link>
          <router-link to="/factory" class="nav-link">
            Factories
            <span v-if="totalFactories > 0" class="nav-badge">{{ totalFactories }}</span>
          </router-link>
          <router-link to="/logistics" class="nav-link">
            Logistics
            <span v-if="totalLogisticsLines > 0" class="nav-badge">{{ totalLogisticsLines }}</span>
          </router-link>
          <router-link to="/blueprints" class="nav-link">Blueprints</router-link>
        </div>
      </nav>

      <main class="main-content">
        <ErrorBoundary fallback="minimal">
          <router-view />
        </ErrorBoundary>
      </main>

      <ToastContainer />
    </ErrorBoundary>
  </div>
</template>

<style scoped>
#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background-color: var(--color-bg-primary, #1a1a1a);
  color: var(--color-text-primary, #e5e5e5);
  transition: background-color var(--transition-normal, 200ms),
              color var(--transition-normal, 200ms);
}

.main-nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  background-color: var(--color-nav-background, #1a1a1a);
  color: var(--color-nav-text, #e5e5e5);
  border-bottom: 1px solid var(--color-border, #404040);
  box-shadow: var(--shadow-sm);
  position: sticky;
  top: 0;
  z-index: var(--z-index-sticky, 1020);
}

.nav-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-ficsit-orange, #f58b00);
  letter-spacing: -0.01em;
}

.nav-links {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.nav-link {
  color: var(--color-nav-text, #e5e5e5);
  text-decoration: none;
  padding: 0.5rem 1rem;
  border-radius: var(--border-radius-sm, 3px);
  transition: all var(--transition-normal, 200ms);
  display: flex;
  align-items: center;
  gap: 0.5rem;
  border: 1px solid transparent;
  font-weight: var(--font-weight-medium, 500);
}

.nav-link:hover {
  background-color: var(--color-nav-hover, #2d2d2d);
  border-color: var(--color-border, #404040);
}

.nav-link.router-link-active {
  background-color: var(--color-surface, #252525);
  border-color: var(--color-ficsit-orange, #f58b00);
  color: var(--color-ficsit-orange, #f58b00);
  box-shadow: var(--shadow-inset-light);
}

.nav-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 1.25rem;
  height: 1.25rem;
  padding: 0 0.375rem;
  font-size: 0.75rem;
  font-weight: var(--font-weight-semibold, 600);
  line-height: 1;
  color: var(--color-text-primary, #e5e5e5);
  background-color: var(--color-ficsit-orange, #f58b00);
  border-radius: 0.625rem;
  border: 1px solid transparent;
  font-family: var(--font-family-mono, monospace);
}

.nav-link.router-link-active .nav-badge {
  background-color: var(--color-text-primary, #e5e5e5);
  color: var(--color-ficsit-orange, #f58b00);
}

.main-content {
  flex: 1;
  padding: 0;
  background-color: var(--color-bg-primary, #1a1a1a);
}
</style>
