@Component
struct Counter {
  @State count: number = 0

  build() {
    Column() {
      Text(`Count: ${this.count}`)
        .fontSize(24)

      Button('Increment')
        .onClick(() => {
          this.count++
        })
    }
  }
}
