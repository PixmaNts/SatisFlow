<template>
  <div id="app">
    <nav class="navbar">
      <div class="nav-brand">
        <h1>SatisFlow</h1>
      </div>
      <div class="nav-links">
        <router-link to="/">Home</router-link>
        <router-link to="/dashboard">Dashboard</router-link>
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
  padding: 1rem 2rem;
  background-color: #2c3e50;
  color: white;
}

.nav-brand h1 {
  margin: 0;
  font-size: 1.5rem;
}

.nav-links {
  display: flex;
  gap: 1rem;
}

.nav-links a {
  color: white;
  text-decoration: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.nav-links a:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.nav-links a.router-link-active {
  background-color: #3498db;
}

main {
  padding: 2rem;
  min-height: calc(100vh - 80px);
}

#app {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  background-color: #f5f5f5;
}
</style>