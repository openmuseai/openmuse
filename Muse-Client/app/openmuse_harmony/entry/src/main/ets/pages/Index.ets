import { LegalWelcome } from './LegalWelcome';

@Entry
@Component
struct Index {
  build() {
    LegalWelcome() {
    Scroll() {
      Column({ space: 16 }) {
        Image($r('app.media.app_icon'))
          .width(80)
          .height(80)
          .borderRadius(18)

        Text($r('app.string.app_name'))
          .fontSize(28)
          .fontWeight(FontWeight.Bold)
          .fontColor('#172033')

        Text($r('app.string.welcome'))
          .fontSize(17)
          .fontColor('#475569')

        Text($r('app.string.bootstrap_notice'))
          .fontSize(13)
          .fontColor('#64748B')
          .textAlign(TextAlign.Center)

        Text($r('app.string.filing_placeholder'))
          .fontSize(12)
          .fontColor('#64748B')
          .textAlign(TextAlign.Center)

        Text($r('app.string.support_email'))
          .fontSize(12)
          .fontColor('#64748B')
          .textAlign(TextAlign.Center)
      }
      .width('100%')
      .padding(16)
      .justifyContent(FlexAlign.Center)
      .alignItems(HorizontalAlign.Center)
    }
    .width('100%')
    .height('100%')
    .backgroundColor('#F7F8FC')
    }
  }
}
