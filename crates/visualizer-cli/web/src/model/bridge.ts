import { EventEmitter } from "@hediet/std/events";
import { VisualizationId, VisualizationData } from "@hediet/visualization-core";

export class Bridge {
	private readonly onMessageEmitter = new EventEmitter<{
		message: IncomingMessage;
	}>();
	public readonly onMessage = this.onMessageEmitter.asEvent();

	private get extendedWindow(): ExtendedWindow {
		return (window as unknown) as ExtendedWindow;
	}

	private _sendMessage: (message: unknown) => void;

	constructor() {
		this.extendedWindow.processEvent = (event) => {
			const message = event as IncomingMessage;
			this.onMessageEmitter.emit({ message });
		};

		let queue: unknown[] = [];

		if (this.extendedWindow.sendMessage) {
			this._sendMessage = this.extendedWindow.sendMessage;
		} else {
			this._sendMessage = (message: unknown) => {
				queue.push(message);
			};

			Object.defineProperty(this.extendedWindow, "sendMessage", {
				get: () => this._sendMessage,
				set: (value) => {
					this._sendMessage = value;
					for (const item of queue) {
						this._sendMessage(item);
					}
					queue.length = 0;
				},
			});
		}
	}

	public sendMessage(message: OutgoingMessage): void {
		this._sendMessage(message);
	}
}

interface ExtendedWindow {
	processEvent: ((event: unknown) => void) | undefined;
	sendMessage: ((message: unknown) => void) | undefined;
}

type OutgoingMessage = { kind: "initialized" };

type IncomingMessage =
	| {
			kind: "showVisualization";
			data: VisualizationData;
	  }
	| {
			kind: "setTheme";
			theme: "light" | "dark";
			requestId: string;
	  };
