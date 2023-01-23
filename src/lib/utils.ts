export function toMetric(bytes: number | null, precision = 1): string {
  if (!bytes) {
    return "0B"
  }
  const units = ["B", "kB", "MB", "GB", "TB", "PB", "EB"]
  let unitIndex = 0
  while (bytes > 1000) {
    bytes /= 1024
    unitIndex++
  }
  return bytes.toFixed(precision) + units[unitIndex]
}

export function calcStrokeWidth(numArcs: number): number {
  if (numArcs === 1) {
    return 12
  }
  if (numArcs < 4) {
    return 10
  }
  if (numArcs === 4) {
    return 8
  }
  if (numArcs === 5) {
    return 6
  }
  if (numArcs === 6) {
    return 5
  }
  if (numArcs < 9) {
    return 4
  }
  return 3
}

export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

export function saturatedPush<T>(arr: Array<T>, element: T, limit: number) {
  arr.push(element)
  while (arr.length > limit) {
    arr.shift()
  }
}
