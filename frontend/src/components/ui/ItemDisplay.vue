<template>
  <div class="item-display" :class="classes">
    <img
      v-if="showIcon"
      :src="iconPath"
      :alt="itemName"
      class="item-icon"
      @error="handleImageError"
    />
    <span v-if="showName" class="item-name">{{ itemName }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { Item } from '@/api/types'
import { useItemIcon } from '@/composables/useItemIcon'

interface Props {
  /** The item to display */
  item: Item | string
  /** Whether to show the icon */
  showIcon?: boolean
  /** Whether to show the name */
  showName?: boolean
  /** Size variant */
  size?: 'sm' | 'md' | 'lg'
  /** Additional CSS classes */
  class?: string
}

const props = withDefaults(defineProps<Props>(), {
  showIcon: true,
  showName: true,
  size: 'md'
})

const { formatItemName: formatName, getItemIconPath: getIconPath } = useItemIcon()

const itemName = computed(() => formatName(props.item))
const iconPath = computed(() => getIconPath(props.item))
const imageError = ref(false)

const showIcon = computed(() => props.showIcon && !imageError.value)

const classes = computed(() => {
  return [
    `item-display--${props.size}`,
    props.class
  ].filter(Boolean)
})

const handleImageError = () => {
  imageError.value = true
  if (import.meta.env.DEV) {
    console.warn(`Icon not found for item: ${props.item} (expected: ${iconPath.value})`)
  }
}
</script>

<style scoped lang="scss">
.item-display {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
}

.item-icon {
  display: inline-block;
  object-fit: contain;
  flex-shrink: 0;
}

.item-name {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
}

// Size variants
.item-display--sm {
  .item-icon {
    width: 16px;
    height: 16px;
  }

  .item-name {
    font-size: var(--font-size-sm, 0.875rem);
  }
}

.item-display--md {
  .item-icon {
    width: 24px;
    height: 24px;
  }

  .item-name {
    font-size: var(--font-size-base, 1rem);
  }
}

.item-display--lg {
  .item-icon {
    width: 32px;
    height: 32px;
  }

  .item-name {
    font-size: var(--font-size-lg, 1.125rem);
  }
}
</style>

