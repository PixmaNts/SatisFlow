<template>
  <div id="app">
    <nav class="navbar">
      <div class="nav-brand">
        <h1>SatisFlow</h1>
      </div>
      <div class="nav-links">
        <router-link to="/">Home</router-link>
        <router-link to="/dashboard">Dashboard</router-link>
        <router-link to="/factories">Factories</router-link>
        <router-link to="/logistics">Logistics</router-link>
      </div>
    </nav>
    
    <main>
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
// Component logic here
</script>

<style scoped>
.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-lg) var(--spacing-2xl);
  background: var(--color-bg-card);
  color: var(--color-text-primary);
  border-bottom: 1px solid var(--color-border);
  box-shadow: var(--shadow-sm);
}

.nav-brand h1 {
  margin: 0;
  font-size: var(--font-size-xl);
  font-weight: 700;
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.nav-links {
  display: flex;
  gap: var(--spacing-sm);
}

.nav-links a {
  color: var(--color-text-secondary);
  text-decoration: none;
  padding: var(--spacing-md) var(--spacing-lg);
  border-radius: var(--radius-md);
  transition: all var(--transition-normal);
  font-weight: 500;
  font-size: var(--font-size-md);
}

.nav-links a:hover {
  background: var(--color-bg-hover);
  color: var(--color-text-primary);
  transform: translateY(-1px);
}

.nav-links a.router-link-active {
  background: var(--gradient-primary);
  color: white;
  box-shadow: var(--shadow-orange);
}

main {
  padding: var(--spacing-2xl);
  min-height: calc(100vh - 80px);
}

#app {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>