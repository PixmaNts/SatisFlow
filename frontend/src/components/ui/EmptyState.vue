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

// Size classes
const sizeClasses = computed(() => {
  const sizes = {
    sm: 'p-4',
    md: 'p-6',
    lg: 'p-8'
  }
  return sizes[props.size]
})

// Icon size classes
const iconSizeClasses = computed(() => {
  const sizes = {
    sm: 'w-8 h-8',
    md: 'w-12 h-12',
    lg: 'w-16 h-16'
  }
  return sizes[props.size]
})

// Title size classes
const titleSizeClasses = computed(() => {
  const sizes = {
    sm: 'text-lg',
    md: 'text-xl',
    lg: 'text-2xl'
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
    <div v-if="illustration" class="mb-6">
      <img
        :src="illustration"
        :alt="title"
        class="max-w-xs mx-auto"
      />
    </div>

    <div v-else-if="icon" class="mb-6">
      <div
        :class="[
          'flex items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800',
          iconSizeClasses
        ]"
        v-html="icon"
      />
    </div>

    <!-- Title -->
    <h3
      :class="[
        titleSizeClasses,
        'font-medium text-gray-900 dark:text-white mb-2'
      ]"
    >
      {{ title }}
    </h3>

    <!-- Description -->
    <p
      v-if="description"
      :class="[
        descriptionSizeClasses,
        'text-gray-500 dark:text-gray-400 mb-6 max-w-md'
      ]"
    >
      {{ description }}
    </p>

    <!-- Action Button -->
    <button
      v-if="actionText"
      :class="[
        'px-4 py-2 rounded-md font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500',
        actionClasses
      ]"
      @click="handleAction"
    >
      {{ actionText }}
    </button>

    <!-- Slot for additional content -->
    <div v-if="$slots.default" class="mt-6">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.empty-state {
  min-height: 200px;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  .empty-state {
    color: var(--color-text-primary);
  }
}

/* Animation for empty state appearance */
.empty-state {
  animation: fade-in 0.3s ease-out;
}

@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(10px);
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
