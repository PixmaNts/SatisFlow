<script setup lang="ts">
import SkeletonLoader from './SkeletonLoader.vue'

interface Props {
  showAvatar?: boolean
  showTitle?: boolean
  showSubtitle?: boolean
  showText?: boolean
  textLines?: number
  showFooter?: boolean
  avatarSize?: number
  titleWidth?: string | number
  subtitleWidth?: string | number
  textWidth?: string | number
  footerWidth?: string | number
}

withDefaults(defineProps<Props>(), {
  showAvatar: false,
  showTitle: true,
  showSubtitle: false,
  showText: true,
  textLines: 3,
  showFooter: false,
  avatarSize: 40,
  titleWidth: '60%',
  subtitleWidth: '40%',
  textWidth: '100%',
  footerWidth: '80%'
})
</script>

<template>
  <div class="skeleton-card">
    <!-- Avatar -->
    <SkeletonLoader
      v-if="showAvatar"
      :width="avatarSize"
      :height="avatarSize"
      variant="circular"
      class="skeleton-avatar"
    />

    <!-- Title -->
    <SkeletonLoader
      v-if="showTitle"
      :width="titleWidth"
      height="1.2em"
      variant="text"
      class="skeleton-title"
    />

    <!-- Subtitle -->
    <SkeletonLoader
      v-if="showSubtitle"
      :width="subtitleWidth"
      height="1em"
      variant="text"
      class="skeleton-subtitle"
    />

    <!-- Text lines -->
    <div v-if="showText" class="skeleton-text-container">
      <SkeletonLoader
        v-for="i in textLines"
        :key="i"
        :width="i === textLines ? '80%' : textWidth"
        height="1em"
        variant="text"
        class="skeleton-text-line"
      />
    </div>

    <!-- Footer -->
    <SkeletonLoader
      v-if="showFooter"
      :width="footerWidth"
      height="1em"
      variant="text"
      class="skeleton-footer"
    />
  </div>
</template>

<style scoped>
.skeleton-card {
  padding: 1rem;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 8px;
  background-color: var(--color-background, #ffffff);
}

.skeleton-avatar {
  margin-bottom: 0.75rem;
}

.skeleton-title {
  margin-bottom: 0.5rem;
}

.skeleton-subtitle {
  margin-bottom: 0.75rem;
}

.skeleton-text-container {
  margin-bottom: 0.75rem;
}

.skeleton-text-line {
  margin-bottom: 0.25rem;
}

.skeleton-footer {
  margin-top: auto;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .skeleton-card {
    border-color: var(--color-border-dark, #374151);
    background-color: var(--color-background-dark, #1f2937);
  }
}
</style>
