import { computed, observable, runInAction } from "mobx";
import { Bridge } from "./bridge";
import {
	Visualization,
	globalVisualizationFactory,
	Theme,
} from "@hediet/visualization-core";
import "./visualizations";

export class Model {
	private readonly api = new Bridge();

	@observable.ref visualization: Visualization | undefined;
	@observable overlayText: string | undefined;

	@observable themeName: "light" | "dark" = "light";

	@computed get theme(): Theme {
		if (this.themeName === "light") {
			return Theme.light;
		} else {
			return Theme.dark;
		}
	}

	private visualizationGotReady = () => {};
	public handleVisualizationGotReady() {
		this.visualizationGotReady();
	}

	constructor() {
		this.api.onMessage.sub(({ message }) => {
			runInAction(() => {
				if (message.kind === "showVisualization") {
					const visualizations = globalVisualizationFactory.getVisualizations(
						message.data,
						undefined
					);
					this.visualization = visualizations.bestVisualization;
				} else if (message.kind === "setTheme") {
					this.themeName = message.theme;
				}
			});
		});

		this.api.sendMessage({
			kind: "initialized",
		});
	}
}
