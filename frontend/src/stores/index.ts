/**
 * Pinia Stores Index
 *
 * This file exports all Pinia stores for convenient importing.
 * Use this file to import all stores at once:
 *
 * import {
 *   useFactoryStore,
 *   useLogisticsStore,
 *   useDashboardStore,
 *   useGameDataStore,
 *   usePreferencesStore
 * } from '@/stores';
 */

// Import stores for re-export
import { useFactoryStore } from './factory'
import { useLogisticsStore } from './logistics'
import { useDashboardStore } from './dashboard'
import { useGameDataStore } from './gameData'
import { usePreferencesStore } from './preferences'

// Export individual stores
export { useFactoryStore, useLogisticsStore, useDashboardStore, useGameDataStore, usePreferencesStore }

// Export all stores as a group for convenience
export const stores = {
  useFactoryStore,
  useLogisticsStore,
  useDashboardStore,
  useGameDataStore,
  usePreferencesStore
}

// Default export with all stores
export default stores
