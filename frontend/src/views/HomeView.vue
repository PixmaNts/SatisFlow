<template>
  <div class="home">
    <div class="hero">
      <h1>SatisFlow</h1>
      <p class="subtitle">Track and optimize your Satisfactory factory production chains</p>
      
      <div class="cta-buttons">
        <button @click="startTracking" class="btn btn-primary">
          Start Tracking
        </button>
        <button @click="tryDemo" class="btn btn-secondary">
          Try Demo
        </button>
        <input 
          type="file" 
          @change="handleFileUpload" 
          accept=".sfactory,.json" 
          ref="fileInput"
          style="display: none"
        />
        <button @click="loadFromFile" class="btn btn-secondary">
          Load from File
        </button>
      </div>
    </div>

    <div class="features">
      <div class="feature">
        <h3>Track Production</h3>
        <p>Monitor production rates across multiple factories</p>
      </div>
      <div class="feature">
        <h3>Identify Bottlenecks</h3>
        <p>Find resource surpluses and shortages</p>
      </div>
      <div class="feature">
        <h3>Optimize Logistics</h3>
        <p>Track material flows between factories</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { importJson, loadSample } from '../lib/tracker'

const router = useRouter()
const fileInput = ref<HTMLInputElement>()

const startTracking = () => {
  router.push('/dashboard')
}

const loadFromFile = () => {
  fileInput.value?.click()
}

const handleFileUpload = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  
  if (file) {
    const reader = new FileReader()
    reader.onload = async (e) => {
      try {
        const content = e.target?.result as string
        await importJson(content)
        router.push('/dashboard')
      } catch (error) {
        console.error('Failed to import factory file:', error)
        alert('Failed to load factory file. Please check the file format.')
      }
    }
    reader.readAsText(file)
  }
}

const tryDemo = async () => {
  await loadSample()
  router.push('/dashboard')
}
</script>

<style scoped>
.home {
  max-width: 1200px;
  margin: 0 auto;
  text-align: center;
}

.hero {
  padding: 4rem 2rem;
}

.hero h1 {
  font-size: 3.5rem;
  color: #2c3e50;
  margin-bottom: 1rem;
}

.subtitle {
  font-size: 1.25rem;
  color: #666;
  margin-bottom: 2rem;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.cta-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
}

.btn {
  padding: 1rem 2rem;
  font-size: 1.1rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: 600;
}

.btn-primary {
  background-color: #3498db;
  color: white;
}

.btn-primary:hover {
  background-color: #2980b9;
  transform: translateY(-2px);
}

.btn-secondary {
  background-color: white;
  color: #3498db;
  border: 2px solid #3498db;
}

.btn-secondary:hover {
  background-color: #3498db;
  color: white;
  transform: translateY(-2px);
}

.features {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  padding: 4rem 2rem;
}

.feature {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s ease;
}

.feature:hover {
  transform: translateY(-4px);
}

.feature h3 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.feature p {
  color: #666;
  line-height: 1.6;
}
</style>
