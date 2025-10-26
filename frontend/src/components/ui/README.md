# Satisflow UI Components

This directory contains reusable UI components for the Satisflow application. All components are built with Vue 3 Composition API, TypeScript, and SCSS with CSS variables for theming.

## Components

### Button
A versatile button component with multiple variants and states.

```vue
<template>
  <Button variant="primary" size="md" @click="handleClick">
    Click me
  </Button>
  
  <Button variant="danger" :loading="isLoading" :disabled="isDisabled">
    Delete
  </Button>
</template>

<script setup>
import { Button } from '@/components/ui'
</script>
```

**Props:**
- `variant`: 'primary' | 'secondary' | 'danger' (default: 'primary')
- `size`: 'sm' | 'md' | 'lg' (default: 'md')
- `disabled`: boolean (default: false)
- `loading`: boolean (default: false)

**Events:**
- `click`: Emitted when button is clicked

### LoadingSpinner
A loading indicator with customizable size and color.

```vue
<template>
  <LoadingSpinner size="lg" color="#3b82f6" />
</template>

<script setup>
import { LoadingSpinner } from '@/components/ui'
</script>
```

**Props:**
- `size`: 'sm' | 'md' | 'lg' (default: 'md')
- `color`: string (default: uses CSS variable)
- `ariaLabel`: string (default: 'Loading')
- `showLabel`: boolean (default: true)

### Modal
A modal dialog component with focus management and accessibility features.

```vue
<template>
  <Modal v-model:show="isModalOpen" title="Confirm Action" @close="handleClose">
    <p>Are you sure you want to proceed?</p>
    
    <template #footer>
      <Button variant="secondary" @click="handleClose">Cancel</Button>
      <Button variant="primary" @click="handleConfirm">Confirm</Button>
    </template>
  </Modal>
</template>

<script setup>
import { ref } from 'vue'
import { Modal, Button } from '@/components/ui'

const isModalOpen = ref(false)

const handleClose = () => {
  isModalOpen.value = false
}

const handleConfirm = () => {
  // Handle confirmation
  handleClose()
}
</script>
```

**Props:**
- `show`: boolean (required)
- `title`: string (optional)
- `closable`: boolean (default: true)
- `closeOnOverlay`: boolean (default: true)

**Events:**
- `close`: Emitted when modal is closed
- `confirm`: Emitted when confirm action is triggered
- `update:show`: Emitted for v-model binding

**Slots:**
- `default`: Modal content
- `footer`: Modal footer actions

### Alert
An alert/notification component with different types and auto-dismiss functionality.

```vue
<template>
  <Alert type="success" dismissible :timeout="5000" @dismiss="handleDismiss">
    Operation completed successfully!
  </Alert>
  
  <Alert type="error">
    An error occurred while processing your request.
  </Alert>
</template>

<script setup>
import { Alert } from '@/components/ui'

const handleDismiss = () => {
  console.log('Alert dismissed')
}
</script>
```

**Props:**
- `type`: 'info' | 'success' | 'warning' | 'error' (default: 'info')
- `dismissible`: boolean (default: false)
- `timeout`: number (default: 0, no auto-dismiss)
- `closeAriaLabel`: string (default: 'Dismiss alert')

**Events:**
- `dismiss`: Emitted when alert is dismissed

**Slots:**
- `default`: Alert content

### Tabs & TabPanel
Tab navigation component with keyboard navigation and accessibility.

```vue
<template>
  <Tabs :tabs="tabs" v-model:activeTab="activeTab" @tab-change="handleTabChange">
    <TabPanel tab-id="overview">
      <h2>Overview</h2>
      <p>Overview content goes here...</p>
    </TabPanel>
    
    <TabPanel tab-id="details">
      <h2>Details</h2>
      <p>Details content goes here...</p>
    </TabPanel>
    
    <TabPanel tab-id="settings">
      <h2>Settings</h2>
      <p>Settings content goes here...</p>
    </TabPanel>
  </Tabs>
</template>

<script setup>
import { ref } from 'vue'
import { Tabs, TabPanel } from '@/components/ui'

const activeTab = ref('overview')

const tabs = [
  { id: 'overview', label: 'Overview' },
  { id: 'details', label: 'Details' },
  { id: 'settings', label: 'Settings' },
]

const handleTabChange = (tabId) => {
  console.log('Active tab:', tabId)
}
</script>
```

**Tabs Props:**
- `tabs`: Array of Tab objects (required)
- `activeTab`: string (optional)
- `ariaLabel`: string (default: 'Tabs')

**Tabs Events:**
- `tab-change`: Emitted when active tab changes
- `update:activeTab`: Emitted for v-model binding

**TabPanel Props:**
- `tabId`: string (required)

### DataTable
A sortable and filterable table component with pagination support.

```vue
<template>
  <DataTable
    :columns="columns"
    :data="data"
    :loading="isLoading"
    filterable
    paginated
    :page-size="10"
    @row-click="handleRowClick"
    @sort-change="handleSortChange"
  >
    <template #cell-status="{ row, value }">
      <span :class="getStatusClass(value)">{{ value }}</span>
    </template>
    
    <template #cell-actions="{ row }">
      <Button variant="secondary" size="sm" @click="editRow(row)">
        Edit
      </Button>
    </template>
  </DataTable>
</template>

<script setup>
import { ref } from 'vue'
import { DataTable, Button } from '@/components/ui'

const isLoading = ref(false)
const data = ref([
  { id: 1, name: 'Item 1', status: 'Active', quantity: 10 },
  { id: 2, name: 'Item 2', status: 'Inactive', quantity: 5 },
  // ... more data
])

const columns = [
  { key: 'id', label: 'ID', sortable: true },
  { key: 'name', label: 'Name', sortable: true },
  { key: 'status', label: 'Status', sortable: true },
  { key: 'quantity', label: 'Quantity', sortable: true },
  { key: 'actions', label: 'Actions', sortable: false },
]

const handleRowClick = (row, index) => {
  console.log('Row clicked:', row, index)
}

const handleSortChange = ({ key, direction }) => {
  console.log('Sort changed:', key, direction)
}

const getStatusClass = (status) => {
  return status === 'Active' ? 'text-green-600' : 'text-red-600'
}
</script>
```

**DataTable Props:**
- `columns`: Array of Column objects (required)
- `data`: Array of data objects (required)
- `sortable`: boolean (default: true)
- `filterable`: boolean (default: false)
- `loading`: boolean (default: false)
- `filterPlaceholder`: string (default: 'Filter...')
- `emptyText`: string (default: 'No data available')
- `paginated`: boolean (default: false)
- `pageSize`: number (default: 10)
- `rowKey`: Function (default: uses index)
- `rowClass`: string | Function (optional)

**DataTable Events:**
- `row-click`: Emitted when a row is clicked
- `sort-change`: Emitted when sort changes

**DataTable Slots:**
- `controls`: Additional table controls
- `cell-{key}`: Custom cell content for specific columns

## Styling

All components use CSS variables defined in `src/assets/styles/variables.css`. You can customize the theme by modifying these variables or overriding them in your application.

### Key CSS Variables

- Colors: `--color-primary-600`, `--color-gray-500`, etc.
- Spacing: `--spacing-sm`, `--spacing-md`, etc.
- Typography: `--font-size-base`, `--font-weight-medium`, etc.
- Border radius: `--border-radius-md`, `--border-radius-lg`, etc.
- Shadows: `--shadow-sm`, `--shadow-md`, etc.

## Accessibility

All components follow accessibility best practices:
- Proper ARIA attributes
- Keyboard navigation support
- Focus management
- Screen reader compatibility

## Usage

Import components individually or use the index file:

```typescript
// Individual imports
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'

// Or import from index
import { Button, Modal, Alert } from '@/components/ui'