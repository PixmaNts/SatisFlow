<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useTheme } from '@/composables/useTheme'
import { registerCommonShortcuts } from '@/composables/useKeyboardShortcuts'
import { ErrorBoundary, ToastContainer } from '@/components/ui'
import SettingsModal from '@/components/SettingsModal.vue'

const { effectiveTheme } = useTheme()
const showSettings = ref(false)

const openSettings = () => {
  showSettings.value = true
}

const closeSettings = () => {
  showSettings.value = false
}

// Set up keyboard shortcuts (only if in browser environment)
if (typeof window !== 'undefined' && window.document) {
  onMounted(() => {
    const unregisterShortcuts = registerCommonShortcuts()

    // Handle custom events from shortcuts
    const handleOpenSettings = () => openSettings()
    const handleEscape = () => {
      if (showSettings.value) {
        closeSettings()
      }
    }

    document.addEventListener('app-open-settings', handleOpenSettings)
    document.addEventListener('app-escape', handleEscape)

    // Clean up on unmount
    onUnmounted(() => {
      unregisterShortcuts()
      document.removeEventListener('app-open-settings', handleOpenSettings)
      document.removeEventListener('app-escape', handleEscape)
    })
  })
}
</script>

<template>
  <div id="app" :class="{ 'dark-theme': effectiveTheme === 'dark', 'light-theme': effectiveTheme === 'light' }">
    <ErrorBoundary fallback="default">
      <nav class="main-nav">
        <h1 class="nav-title">Satisflow</h1>
        <div class="nav-links">
          <router-link to="/" class="nav-link">Dashboard</router-link>
          <router-link to="/factory" class="nav-link">Factory</router-link>
          <router-link to="/logistics" class="nav-link">Logistics</router-link>
          <router-link to="/blueprints" class="nav-link">Blueprints</router-link>
          <button class="nav-link settings-button" @click="openSettings" title="Settings">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 15C13.6569 15 15 13.6569 15 12C15 10.3431 13.6569 9 12 9C10.3431 9 9 10.3431 9 12C9 13.6569 10.3431 15 12 15Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M19.4 15C19.2669 15.3016 19.2272 15.6362 19.286 15.96C19.3448 16.2838 19.4983 16.5816 19.73 16.81L19.79 16.87C19.976 17.0557 20.1235 17.276 20.2241 17.5181C20.3248 17.7602 20.3766 18.0193 20.3766 18.281C20.3766 18.5427 20.3248 18.8019 20.2241 19.044C20.1235 19.2861 19.976 19.5064 19.79 19.692C19.6043 19.878 19.384 20.0255 19.1419 20.1261C18.8998 20.2268 18.6407 20.2786 18.379 20.2786C18.1173 20.2786 17.8581 20.2268 17.616 20.1261C17.3739 20.0255 17.1536 19.878 16.968 19.692L16.908 19.632C16.6796 19.4003 16.3818 19.2468 16.058 19.188C15.7342 19.1292 15.3996 19.1689 15.098 19.302C14.8034 19.4292 14.554 19.6401 14.3824 19.9079C14.2108 20.1757 14.1248 20.4881 14.136 20.804V21C14.136 21.5304 13.9253 22.0391 13.5502 22.4142C13.1751 22.7893 12.6664 23 12.136 23C11.6056 23 11.0969 22.7893 10.7218 22.4142C10.3467 22.0391 10.136 21.5304 10.136 21V20.91C10.1189 20.5857 9.99872 20.2763 9.793 20.03C9.58728 19.7837 9.30717 19.6133 8.996 19.546C8.67222 19.4872 8.3376 19.5269 8.036 19.66C7.74144 19.7872 7.49204 19.9981 7.32044 20.2659C7.14884 20.5337 7.06284 20.8461 7.074 21.162V21.242C7.074 21.7724 6.86329 22.2811 6.48821 22.6562C6.11314 23.0313 5.60443 23.242 5.074 23.242C4.54357 23.242 4.03486 23.0313 3.65979 22.6562C3.28471 22.2811 3.074 21.7724 3.074 21.242V21.162C3.074 20.8461 2.988 20.5337 2.8164 20.2659C2.6448 19.9981 2.3954 19.7872 2.10084 19.66C1.79924 19.5269 1.46462 19.4872 1.14084 19.546C0.819966 19.606 0.524842 19.7594 0.298 19.99L0.238 20.05C0.0522848 20.236 -0.168018 20.3835 -0.410119 20.4841C-0.652219 20.5848 -0.911385 20.6366 -1.17308 20.6366C-1.43478 20.6366 -1.69395 20.5848 -1.93605 20.4841C-2.17815 20.3835 -2.39845 20.236 -2.58416 20.05C-2.77017 19.8643 -2.91766 19.644 -3.01832 19.4019C-3.11898 19.1598 -3.17079 18.9007 -3.17079 18.639C-3.17079 18.3773 -3.11898 18.1181 -3.01832 17.876C-2.91766 17.6339 -2.77017 17.4136 -2.58416 17.228L-2.52416 17.168C-2.29248 16.9396 -2.13898 16.6418 -2.08016 16.318C-2.02134 15.9942 -2.06104 15.6596 -2.19416 15.358C-2.32136 15.0634 -2.53228 14.814 -2.80007 14.6424C-3.06785 14.4708 -3.38024 14.3848 -3.69616 14.396H-3.78C-4.31043 14.396 -4.81914 14.1853 -5.19421 13.8102C-5.56929 13.4351 -5.78 12.9264 -5.78 12.396C-5.78 11.8656 -5.56929 11.3569 -5.19421 10.9818C-4.81914 10.6067 -4.31043 10.396 -3.78 10.396H-3.69C-3.37572 10.3789 -3.06628 10.2587 -2.82 10.053C-2.57372 9.84728 -2.40333 9.56717 -2.336 9.256C-2.27718 8.93222 -2.31688 8.5976 -2.45 8.296C-2.5772 8.00144 -2.78812 7.75204 -3.05591 7.58044C-3.32369 7.40884 -3.63608 7.32284 -3.952 7.334H-4.032C-4.56243 7.334 -5.07114 7.12329 -5.44621 6.74821C-5.82129 6.37314 -6.032 5.86443 -6.032 5.334C-6.032 4.80357 -5.82129 4.29486 -5.44621 3.91979C-5.07114 3.54471 -4.56243 3.334 -4.032 3.334H-3.952C-3.63608 3.334 -3.32369 3.248 -3.05591 3.0764C-2.78812 2.9048 -2.5772 2.6554 -2.45 2.36084C-2.31688 2.05924 -2.27718 1.72462 -2.336 1.40084C-2.39584 1.07997 -2.53916 0.784842 -2.77 0.558L-2.83 0.498C-3.01601 0.312284 -3.1635 0.0919816 -3.26416 -0.150119C-3.36482 -0.392219 -3.41663 -0.651385 -3.41663 -0.91308C-3.41663 -1.17478 -3.36482 -1.43395 -3.26416 -1.67605C-3.1635 -1.91815 -3.01601 -2.13845 -2.83 -2.32416C-2.64428 -2.51017 -2.42398 -2.65766 -2.18188 -2.75832C-1.93978 -2.85898 -1.68062 -2.91079 -1.41892 -2.91079C-1.15722 -2.91079 -0.898054 -2.85898 -0.655954 -2.75832C-0.413854 -2.65766 -0.193552 -2.51017 -0.00784173 -2.32416L0.0521583 -2.26416C0.280542 -2.03248 0.578322 -1.87898 0.902101 -1.82016C1.22588 -1.76134 1.5605 -1.80104 1.8621 -1.93416H1.9621C2.25666 -2.06136 2.50606 -2.27228 2.67766 -2.54007C2.84926 -2.80785 2.93526 -3.12024 2.9241 -3.43616V-3.516C2.9241 -4.04643 3.13481 -4.55514 3.50989 -4.93021C3.88496 -5.30529 4.39367 -5.516 4.9241 -5.516C5.45453 -5.516 5.96324 -5.30529 6.33831 -4.93021C6.71339 -4.55514 6.9241 -4.04643 6.9241 -3.516V-3.426C6.93526 -3.11008 7.02126 -2.79769 7.19286 -2.52991C7.36446 -2.26212 7.61386 -2.0512 7.90842 -1.924C8.21002 -1.79088 8.54464 -1.75118 8.86842 -1.812C9.1893 -1.87184 9.48442 -2.01516 9.71126 -2.246L9.77126 -2.306C9.95698 -2.49201 10.1773 -2.6395 10.4194 -2.74016C10.6615 -2.84082 10.9206 -2.89263 11.1823 -2.89263C11.444 -2.89263 11.7032 -2.84082 11.9453 -2.74016C12.1874 -2.6395 12.4077 -2.49201 12.5934 -2.306C12.7794 -2.12028 12.9269 -1.89998 13.0276 -1.65788C13.1282 -1.41578 13.18 -1.15662 13.18 -0.894919C13.18 -0.633219 13.1282 -0.374054 13.0276 -0.131954C12.9269 0.110146 12.7794 0.330448 12.5934 0.516158L12.5334 0.576158C12.3017 0.804542 12.1482 1.10232 12.0894 1.4261C12.0306 1.74988 12.0703 2.0845 12.2034 2.3861V2.4861C12.3306 2.78066 12.5415 3.03006 12.8093 3.20166C13.0771 3.37326 13.3895 3.45926 13.7054 3.4481H13.7854C14.3158 3.4481 14.8245 3.65881 15.1996 4.03389C15.5747 4.40896 15.7854 4.91767 15.7854 5.4481C15.7854 5.97853 15.5747 6.48724 15.1996 6.86231C14.8245 7.23739 14.3158 7.4481 13.7854 7.4481H13.6954C13.3795 7.45926 13.0671 7.54526 12.7993 7.71686C12.5315 7.88846 12.3206 8.13786 12.1934 8.43242V8.43242Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
      </nav>

      <main class="main-content">
        <ErrorBoundary fallback="minimal">
          <router-view />
        </ErrorBoundary>
      </main>

      <SettingsModal :open="showSettings" @close="closeSettings" />
      <ToastContainer />
    </ErrorBoundary>
  </div>
</template>

<style scoped>
#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background-color: var(--color-background, #ffffff);
  color: var(--color-text-primary, #111827);
  transition: background-color var(--transition-normal, 300ms ease-in-out),
              color var(--transition-normal, 300ms ease-in-out);
}

.main-nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  background-color: var(--color-nav-background, #2c3e50);
  color: var(--color-nav-text, #ffffff);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.nav-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: bold;
}

.nav-links {
  display: flex;
  gap: 1.5rem;
  align-items: center;
}

.nav-link {
  color: var(--color-nav-text, #ffffff);
  text-decoration: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  transition: background-color 0.3s;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.nav-link:hover,
.nav-link.router-link-active {
  background-color: var(--color-nav-hover, #34495e);
}

.settings-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 4px;
  transition: background-color 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.settings-button:hover {
  background-color: var(--color-nav-hover, #34495e);
}

.main-content {
  flex: 1;
  padding: 0;
}
</style>
