<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  icon?: string
  title: string
  description?: string
  actionText?: string
  actionVariant?: 'primary' | 'secondary'
  size?: 'sm' | 'md' | 'lg'
  centered?: boolean
  illustration?: string
}

const props = withDefaults(defineProps<Props>(), {
  actionVariant: 'primary',
  size: 'md',
  centered: true
})

const emit = defineEmits<{
  action: []
}>()

// Icon components map
const iconMap: Record<string, string> = {
  blueprint: `
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M9 2v6m6-6v6M9 18v4m6-4v4M3 10H1m2 6H1m20-6h-2m2 6h-2M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
      <path d="M9 10h6v6H9V10z"/>
    </svg>
  `
}

// Size classes
const sizeClasses = computed(() => {
  const sizes = {
    sm: 'p-4',
    md: 'p-8',
    lg: 'p-12'
  }
  return sizes[props.size]
})

// Icon size classes
const iconSizeClasses = computed(() => {
  const sizes = {
    sm: 'w-8 h-8',
    md: 'w-10 h-10',
    lg: 'w-12 h-12'
  }
  return sizes[props.size]
})

// Title size classes
const titleSizeClasses = computed(() => {
  const sizes = {
    sm: 'text-lg',
    md: 'text-2xl',
    lg: 'text-3xl'
  }
  return sizes[props.size]
})

// Description size classes
const descriptionSizeClasses = computed(() => {
  const sizes = {
    sm: 'text-sm',
    md: 'text-base',
    lg: 'text-lg'
  }
  return sizes[props.size]
})

// Action button classes
const actionClasses = computed(() => {
  const variants = {
    primary: 'bg-blue-600 hover:bg-blue-700 text-white',
    secondary: 'bg-gray-600 hover:bg-gray-700 text-white'
  }
  return variants[props.actionVariant]
})

// Get icon HTML
const iconHtml = computed(() => {
  if (!props.icon) return null
  return iconMap[props.icon] || props.icon
})

// Handle action click
const handleAction = () => {
  emit('action')
}
</script>

<template>
  <div
    :class="[
      'empty-state',
      sizeClasses,
      centered ? 'text-center' : 'text-left',
      'flex flex-col items-center justify-center'
    ]"
    role="presentation"
  >
    <!-- Illustration or Icon -->
    <div v-if="illustration" class="empty-illustration mb-8">
      <img
        :src="illustration"
        :alt="title"
        class="max-w-xs mx-auto"
      />
    </div>

    <div v-else-if="iconHtml" class="empty-icon-wrapper mb-4">
      <div
        :class="[
          'empty-icon',
          iconSizeClasses
        ]"
        v-html="iconHtml"
      />
    </div>

    <!-- Title -->
    <h3
      :class="[
        titleSizeClasses,
        'empty-title font-semibold text-gray-900 dark:text-white mb-3'
      ]"
    >
      {{ title }}
    </h3>

    <!-- Description -->
    <p
      v-if="description"
      :class="[
        descriptionSizeClasses,
        'empty-description text-gray-500 dark:text-gray-400 mb-8 max-w-lg'
      ]"
    >
      {{ description }}
    </p>

    <!-- Action Button -->
    <button
      v-if="actionText"
      :class="[
        'px-6 py-3 rounded-lg font-medium transition-all focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 shadow-sm hover:shadow-md',
        actionClasses
      ]"
      @click="handleAction"
    >
      {{ actionText }}
    </button>

    <!-- Slot for additional content (actions) -->
    <div v-if="$slots.actions || $slots.default" class="empty-actions mt-2">
      <slot name="actions" />
      <slot />
    </div>
  </div>
</template>

<style scoped>
.empty-state {
  min-height: 300px;
}

.empty-icon-wrapper {
  opacity: 0.25;
  color: var(--color-text-muted, #8a8a8a);
  margin-bottom: 0.75rem;
}

.empty-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  color: currentColor;
}

.empty-icon :deep(svg) {
  width: 100%;
  height: 100%;
  opacity: 0.4;
}

.empty-title {
  color: var(--color-text-primary, #e5e5e5);
}

.empty-description {
  color: var(--color-text-secondary, #b8b8b8);
  line-height: 1.6;
}

.empty-actions {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
}

.empty-illustration {
  opacity: 0.7;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  .empty-state {
    color: var(--color-text-primary);
  }
}

/* Animation for empty state appearance */
.empty-state {
  animation: fade-in 0.4s ease-out;
}

@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Focus styles for accessibility */
button:focus-visible {
  outline: 2px solid currentColor;
  outline-offset: 2px;
}
</style>
