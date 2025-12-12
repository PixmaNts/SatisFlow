import type { Item } from '@/api/types'

/**
 * Converts a camelCase Item type to the corresponding icon filename with underscores.
 * Example: "SteelBeam" -> "Steel_Beam"
 * Example: "IronOre" -> "Iron_Ore"
 * Example: "EncasedIndustrialBeam" -> "Encased_Industrial_Beam"
 * Example: "AIExpansionServer" -> "AI_Expansion_Server"
 */
function itemToIconName(item: Item | string): string {
  // Split camelCase by inserting underscores before capital letters
  // Handle both single capitals and consecutive capitals (like "AI")
  return item
    .replace(/([a-z\d])([A-Z])/g, '$1_$2') // Insert underscore between lowercase/digit and uppercase
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2') // Handle consecutive capitals followed by lowercase (e.g., "AI" + "Expansion")
    .split('_')
    .map(word => {
      // Capitalize first letter, keep rest as-is (handles "AI" -> "AI", "Steel" -> "Steel")
      if (word.length === 0) return word
      return word.charAt(0).toUpperCase() + word.slice(1)
    })
    .join('_')
}

/**
 * Gets the icon path for an item.
 * Returns the path to the icon image file.
 * @param item - The item type (camelCase)
 * @returns The path to the icon image (e.g., "/icons/Steel_Beam.png")
 */
export function getItemIconPath(item: Item | string): string {
  const iconName = itemToIconName(item)
  return `/icons/${iconName}.png`
}

/**
 * Gets the icon URL for an item.
 * Alias for getItemIconPath for consistency.
 * @param item - The item type (camelCase)
 * @returns The URL/path to the icon image
 */
export function getItemIconUrl(item: Item | string): string {
  return getItemIconPath(item)
}

/**
 * Formats an item name for display.
 * Converts camelCase to a readable format with spaces.
 * Example: "SteelBeam" -> "Steel Beam"
 * Example: "IronOre" -> "Iron Ore"
 * @param item - The item type (camelCase) or string
 * @returns Formatted item name
 */
export function formatItemName(item: Item | string): string {
  return item
    .replace(/([a-z])([A-Z])/g, '$1 $2') // Insert space between lowercase and uppercase
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1 $2') // Handle consecutive capitals
    .split(' ')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()) // Capitalize first letter, lowercase rest
    .join(' ')
}

/**
 * Composable for item icon and name utilities.
 * Provides reactive utilities for working with item icons and names.
 */
export function useItemIcon() {
  return {
    getItemIconPath,
    getItemIconUrl,
    formatItemName
  }
}

