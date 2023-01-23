import { writable } from "svelte/store"
import { colord } from "colord"

export const foregroundColor = writable(colord("#ffffff"))
export const backgroundColor = writable(colord("rgba(0, 0, 0, 0.5)"))
export const titleColor = writable(colord("#00ff00"))
export const accentColor = writable(colord("#ff00ff"))
export const arcTrackColor = writable(colord("rgba(255, 255, 255, 0.2)"))
export const arcCapColor = writable(colord("#ffffff"))
export const fontFamily = writable("Inter, Avenir, Helvetica, Arial, sans-serif")
