<template>
  <div class="dashboard">
    <header class="dashboard-header">
      <h1>Factory Dashboard</h1>
      <div class="dashboard-actions">
        <button @click="addFactory" class="btn btn-primary">
          Add Factory
        </button>
        <button @click="exportData" class="btn btn-secondary">
          Export Data
        </button>
        <button @click="loadDemo" class="btn btn-secondary">
          Load Sample Demo
        </button>
      </div>
    </header>

    <div class="dashboard-content">
      <div class="stats-grid">
        <div class="stat-card">
          <h3>Factories</h3>
          <p class="stat-value">{{ factoryCount }}</p>
        </div>
        <div class="stat-card">
          <h3>Production Lines</h3>
          <p class="stat-value">{{ productionLineCount }}</p>
        </div>
        <div class="stat-card">
          <h3>Items Tracked</h3>
          <p class="stat-value">{{ itemCount }}</p>
        </div>
      </div>

      <div class="main-content">
        <div class="factory-list" v-if="factoryCount > 0">
          <h2>Your Factories</h2>
          <p class="placeholder">Factory management interface will go here</p>
        </div>
        
        <div class="welcome-message" v-else>
          <h2>Welcome to SatisFlow!</h2>
          <p>Start by creating your first factory to track production.</p>
          <button @click="addFactory" class="btn btn-primary btn-large">
            Create Your First Factory
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getCounts, exportJson, loadSample } from '../lib/tracker'

const factoryCount = ref(0)
const productionLineCount = ref(0)
const itemCount = ref(0)

const refresh = async () => {
  const c = await getCounts()
  factoryCount.value = c.factories
  productionLineCount.value = c.lines
  itemCount.value = c.items
}

const addFactory = () => {
  // TODO: Implement factory creation dialog wired to WASM
  alert('Factory creation dialog will be implemented')
}

const exportData = async () => {
  const json = await exportJson()
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'satisflow_data.json'
  a.click()
  URL.revokeObjectURL(url)
}

const loadDemo = async () => {
  await loadSample()
  await refresh()
}

onMounted(async () => {
  await refresh()
})
</script>

<style scoped>
.dashboard {
  max-width: 1200px;
  margin: 0 auto;
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.dashboard-header h1 {
  color: #2c3e50;
  margin: 0;
}

.dashboard-actions {
  display: flex;
  gap: 1rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  font-size: 0.9rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: 600;
}

.btn-large {
  padding: 1rem 2rem;
  font-size: 1.1rem;
}

.btn-primary {
  background-color: #3498db;
  color: white;
}

.btn-primary:hover {
  background-color: #2980b9;
}

.btn-secondary {
  background-color: white;
  color: #3498db;
  border: 2px solid #3498db;
}

.btn-secondary:hover {
  background-color: #3498db;
  color: white;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.stat-card h3 {
  color: #666;
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
  color: #2c3e50;
  margin: 0;
}

.main-content {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 2rem;
}

.welcome-message {
  text-align: center;
  padding: 2rem;
}

.welcome-message h2 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.welcome-message p {
  color: #666;
  margin-bottom: 2rem;
  font-size: 1.1rem;
}

.placeholder {
  color: #666;
  font-style: italic;
  text-align: center;
  padding: 2rem;
}
</style>
