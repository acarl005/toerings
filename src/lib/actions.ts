import uPlot from "uplot"
import type { Options } from "uplot"
import { colord, Colord } from "colord"

export function styleVars(node: HTMLElement, props: Record<string, any>) {
  Object.entries(props).forEach(([key, value]) => {
    node.style.setProperty(`--${key}`, value)
  })

  return {
    update(newProps: Record<string, any>) {
      Object.entries(newProps).forEach(([key, value]) => {
        node.style.setProperty(`--${key}`, value)
        delete props[key]
      })

      Object.keys(props).forEach(name => node.style.removeProperty(`--${name}`))
      props = newProps
    }
  }
}

export interface PlotData {
  x: Array<number>
  y: Array<number>
  color?: Colord
}

function makePlotOptions(color: Colord): Options {
  const stroke = color.alpha(0.7).lighten(0.2).toHslString()
  const fill = color.alpha(0.5).lighten(0.2).toHslString()
  return {
    width: 180,
    height: 15,
    pxAlign: false,
    cursor: { show: false },
    select: { show: false },
    legend: { show: false },
    scales: {
      x: { time: false }
    },
    axes: [{ show: false }, { show: false }],
    series: [{}, { stroke, fill }]
  }
}

export function uPlotAction(node: HTMLElement, data: PlotData) {
  const { x, y, color } = data
  const options = makePlotOptions(color || colord("#fff"))
  const plot = new uPlot(options, [x, y], node)
  return {
    update(newData: PlotData) {
      plot.setData([newData.x, newData.y])
      plot.series[1].stroke = () =>
        (newData.color || colord("#fff")).alpha(0.7).lighten(0.2).toHslString()
      plot.series[1].fill = () =>
        (newData.color || colord("#fff")).alpha(0.5).lighten(0.2).toHslString()
    }
  }
}
