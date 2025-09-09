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
        // Add small delay to ensure WASM state is fully updated
        await new Promise(resolve => setTimeout(resolve, 100))
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
  // Add small delay to ensure WASM state is fully updated
  await new Promise(resolve => setTimeout(resolve, 100))
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
  background: var(--gradient-hero);
  border-radius: var(--radius-2xl);
  margin: var(--spacing-2xl);
  border: 1px solid var(--color-border-accent);
  box-shadow: var(--shadow-lg);
}

.hero h1 {
  font-size: 3.5rem;
  color: var(--color-primary);
  margin-bottom: 1rem;
  font-weight: 700;
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.subtitle {
  font-size: 1.25rem;
  color: var(--color-text-secondary);
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
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-normal);
  font-weight: 600;
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.btn-primary {
  background: var(--gradient-primary);
  color: white;
  box-shadow: var(--shadow-orange);
  border: none;
}

.btn-primary:hover {
  background: var(--color-primary-dark);
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg), var(--shadow-orange);
}

.btn-secondary {
  background: var(--color-bg-card);
  color: var(--color-primary);
  border: 2px solid var(--color-primary);
  box-shadow: var(--shadow-sm);
}

.btn-secondary:hover {
  background: var(--color-primary-lighter);
  color: var(--color-primary-dark);
  border-color: var(--color-primary-dark);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.features {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  padding: 4rem 2rem;
}

.feature {
  background: var(--gradient-card);
  border: 1px solid var(--color-border);
  padding: 2rem;
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-md);
  transition: all var(--transition-normal);
}

.feature:hover {
  transform: translateY(-4px);
  border-color: var(--color-primary-light);
  box-shadow: var(--shadow-xl);
  background: var(--color-bg-card);
}

.feature h3 {
  color: var(--color-text-primary);
  margin-bottom: 1rem;
  font-size: var(--font-size-xl);
}

.feature p {
  color: var(--color-text-secondary);
  line-height: 1.6;
}

@media (max-width: 768px) {
  .hero {
    margin: var(--spacing-lg);
    padding: 2rem 1rem;
  }
  
  .hero h1 {
    font-size: 2.5rem;
  }
  
  .cta-buttons {
    flex-direction: column;
    align-items: center;
  }
  
  .btn {
    min-width: 200px;
  }
}
</style>
