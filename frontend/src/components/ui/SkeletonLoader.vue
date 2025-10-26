<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  width?: string | number
  height?: string | number
  class?: string
  variant?: 'text' | 'rectangular' | 'circular'
  animation?: 'pulse' | 'wave' | 'none'
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'text',
  animation: 'pulse'
})

// Computed styles
const computedStyle = computed(() => {
  const style: Record<string, string> = {}

  if (props.width) {
    style.width = typeof props.width === 'number' ? `${props.width}px` : props.width
  }

  if (props.height) {
    style.height = typeof props.height === 'number' ? `${props.height}px` : props.height
  }

  return style
})

// Base classes
const baseClasses = computed(() => {
  const classes = ['skeleton-loader']

  // Variant classes
  switch (props.variant) {
    case 'text':
      classes.push('skeleton-text')
      break
    case 'rectangular':
      classes.push('skeleton-rectangular')
      break
    case 'circular':
      classes.push('skeleton-circular')
      break
  }

  // Animation classes
  switch (props.animation) {
    case 'pulse':
      classes.push('skeleton-pulse')
      break
    case 'wave':
      classes.push('skeleton-wave')
      break
    case 'none':
      classes.push('skeleton-none')
      break
  }

  // Custom classes
  if (props.class) {
    classes.push(props.class)
  }

  return classes.join(' ')
})
</script>

<template>
  <div
    :class="baseClasses"
    :style="computedStyle"
    role="presentation"
    aria-hidden="true"
  />
</template>

<style scoped>
.skeleton-loader {
  background-color: var(--skeleton-base, #e5e7eb);
  border-radius: 4px;
  display: inline-block;
  position: relative;
  overflow: hidden;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .skeleton-loader {
    background-color: var(--skeleton-base-dark, #374151);
  }
}

/* Variants */
.skeleton-text {
  border-radius: 4px;
  height: 1em;
  margin: 0.25em 0;
}

.skeleton-rectangular {
  border-radius: 4px;
}

.skeleton-circular {
  border-radius: 50%;
}

/* Animations */
.skeleton-pulse {
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

@keyframes skeleton-pulse {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0.4;
  }
  100% {
    opacity: 1;
  }
}

.skeleton-wave {
  animation: skeleton-wave 1.6s linear infinite;
  background: linear-gradient(
    90deg,
    var(--skeleton-base, #e5e7eb) 0%,
    var(--skeleton-highlight, #f3f4f6) 50%,
    var(--skeleton-base, #e5e7eb) 100%
  );
  background-size: 200% 100%;
}

@media (prefers-color-scheme: dark) {
  .skeleton-wave {
    background: linear-gradient(
      90deg,
      var(--skeleton-base-dark, #374151) 0%,
      var(--skeleton-highlight-dark, #4b5563) 50%,
      var(--skeleton-base-dark, #374151) 100%
    );
    background-size: 200% 100%;
  }
}

@keyframes skeleton-wave {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

.skeleton-none {
  animation: none;
}

/* Respect prefers-reduced-motion */
@media (prefers-reduced-motion: reduce) {
  .skeleton-pulse,
  .skeleton-wave {
    animation: none;
  }
}
</style>
