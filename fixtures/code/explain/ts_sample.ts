class EventBus {
    private handlers: Map<string, Function[]> = new Map();

    on(event: string, handler: Function): void {
        if (!this.handlers.has(event)) {
            this.handlers.set(event, []);
        }
        this.handlers.get(event)!.push(handler);
    }

    emit(event: string, data: unknown): void {
        this.handlers.get(event)?.forEach(h => h(data));
    }
}
