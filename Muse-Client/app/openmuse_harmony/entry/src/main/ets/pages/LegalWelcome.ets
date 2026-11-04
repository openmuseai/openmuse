import { preferences } from '@kit.ArkData';
import { common } from '@kit.AbilityKit';

const LEGAL_REVISION: string = 'china:2026-11-03';

/** Bundled first-use boundary; invoke application content only after durable consent. */
@Component
export struct LegalWelcome {
  @BuilderParam content: () => void = this.emptyContent;
  @State private restored: boolean = false;
  @State private accepted: boolean = false;
  @State private checked: boolean = false;
  @State private busy: boolean = false;
  @State private failed: boolean = false;
  @State private viewportWidth: number = 390;

  aboutToAppear(): void { this.restoreConsent(); }

  @Builder
  private emptyContent() { Column() {} }

  private async restoreConsent(): Promise<void> {
    try {
      const store = await preferences.getPreferences(getContext(this), { name: 'openmuse_first_use' });
      this.accepted = (await store.get('legalRevision', '')) === LEGAL_REVISION;
    } catch (_) { this.failed = true; }
    this.restored = true;
  }

  private async acceptConsent(): Promise<void> {
    if (!this.checked || this.busy) return;
    this.busy = true;
    this.failed = false;
    try {
      const store = await preferences.getPreferences(getContext(this), { name: 'openmuse_first_use' });
      await store.put('legalRevision', LEGAL_REVISION);
      await store.flush();
      this.accepted = true;
    } catch (_) { this.failed = true; }
    this.busy = false;
  }

  private async openDocument(path: string): Promise<void> {
    try {
      const context = getContext(this) as common.UIAbilityContext;
      await context.startAbility({ action: 'ohos.want.action.viewData', uri: 'https://openmuseai.com/zh/' + path });
    } catch (_) { this.failed = true; }
  }

  @Builder
  private brand() {
    Row({ space: 11 }) {
      Image($r('app.media.app_icon')).width(36).height(36).borderRadius(10)
      Text('OpenMuse').fontSize(21).fontWeight(FontWeight.Medium).fontColor($r('app.color.onboarding_text'))
    }
  }

  @Builder
  private documentRow(title: ResourceStr, detail: ResourceStr, path: string) {
    Column({ space: 5 }) {
      Text(title).fontSize(16).fontWeight(FontWeight.Medium).fontColor($r('app.color.onboarding_text'))
      Text(detail).fontSize(12).fontColor($r('app.color.onboarding_secondary'))
    }.alignItems(HorizontalAlign.Start).width('100%').padding(20)
      .constraintSize({ minHeight: 84 }).focusable(true).enabled(!this.busy)
      .accessibilityGroup(true).accessibilityText(title)
      .onClick(() => { this.openDocument(path); })
  }

  @Builder
  private documents() {
    Column({ space: 0 }) {
      Text(this.viewportWidth >= 960 ? $r('app.string.legal_intro') : $r('app.string.legal_title'))
        .fontSize(28).fontWeight(FontWeight.Medium).fontColor($r('app.color.onboarding_text')).width('100%')
      Text($r('app.string.legal_description')).fontSize(15).lineHeight(24)
        .fontColor($r('app.color.onboarding_secondary')).width('100%').margin({ top: 14 })
      Column({ space: 4 }) {
        Text($r('app.string.legal_region')).fontSize(12).fontColor($r('app.color.onboarding_secondary'))
        Text($r('app.string.legal_region_name')).fontSize(15).fontColor($r('app.color.onboarding_text'))
      }.alignItems(HorizontalAlign.Start).width('100%').margin({ top: 24, bottom: 20 })
      Column() {
        this.documentRow($r('app.string.legal_terms'), $r('app.string.legal_terms_detail'), 'terms')
        Divider().color($r('app.color.onboarding_border'))
        this.documentRow($r('app.string.legal_privacy'), $r('app.string.legal_privacy_detail'), 'privacy')
      }.borderRadius(18).borderWidth(1).borderColor($r('app.color.onboarding_border'))
    }.alignItems(HorizontalAlign.Start).width('100%')
  }

  @Builder
  private decision() {
    Column({ space: 10 }) {
      Row({ space: 10 }) {
        Checkbox({ name: 'legalConsent' }).select(this.checked).enabled(!this.busy)
          .selectedColor($r('app.color.onboarding_primary'))
          .onChange((value: boolean) => { this.checked = value; })
        Text($r('app.string.legal_agree')).fontSize(14).lineHeight(21)
          .fontColor($r('app.color.onboarding_text')).layoutWeight(1)
          .onClick(() => { if (!this.busy) this.checked = !this.checked; })
      }.constraintSize({ minHeight: 48 }).width('100%')
      if (this.failed) {
        Text($r('app.string.legal_error')).fontSize(14).fontColor($r('app.color.onboarding_error')).width('100%')
      }
      Button(this.busy ? $r('app.string.legal_busy') : $r('app.string.legal_accept'), { type: ButtonType.Normal })
        .width('100%').constraintSize({ minHeight: 54 }).borderRadius(14).fontSize(16)
        .backgroundColor(this.checked ? $r('app.color.onboarding_primary') : $r('app.color.onboarding_disabled'))
        .fontColor(this.checked ? $r('app.color.onboarding_on_primary') : $r('app.color.onboarding_secondary'))
        .enabled(this.checked && !this.busy).onClick(() => { this.acceptConsent(); })
      Button($r('app.string.legal_decline'), { type: ButtonType.Normal })
        .backgroundColor(Color.Transparent).fontColor($r('app.color.onboarding_secondary'))
        .constraintSize({ minHeight: 48 }).fontSize(14).enabled(!this.busy)
        .onClick(() => { this.checked = false; })
      Text($r('app.string.legal_decision')).fontSize(12).textAlign(TextAlign.Center)
        .fontColor($r('app.color.onboarding_secondary')).width('100%')
    }.width('100%').padding({ top: 20 })
  }

  build() {
    Column() {
      if (!this.restored) {
        LoadingProgress().width(32).height(32)
      } else if (this.accepted) {
        this.content()
      } else {
        Scroll() {
          Row({ space: this.viewportWidth >= 960 ? 80 : 0 }) {
            if (this.viewportWidth >= 960) {
              Column({ space: 20 }) {
                this.brand()
                Text($r('app.string.legal_title')).fontSize(40).fontWeight(FontWeight.Medium)
                  .fontColor($r('app.color.onboarding_text')).margin({ top: 36 })
                Text($r('app.string.legal_workspace')).fontSize(18).fontColor($r('app.color.onboarding_secondary'))
              }.alignItems(HorizontalAlign.Start).layoutWeight(1)
            }
            Column() {
              if (this.viewportWidth < 960) {
                this.brand()
                Blank().height(34)
              }
              this.documents()
              this.decision()
            }.alignItems(HorizontalAlign.Start).padding(this.viewportWidth >= 960 ? 32 : 24)
              .borderRadius(this.viewportWidth >= 960 ? 24 : 0)
              .borderWidth(this.viewportWidth >= 960 ? 1 : 0).borderColor($r('app.color.onboarding_border'))
              .backgroundColor($r('app.color.onboarding_surface')).layoutWeight(1)
          }.padding(this.viewportWidth >= 960 ? 48 : 0).constraintSize({ maxWidth: 1200 })
        }.width('100%').height('100%')
      }
    }.width('100%').height('100%').justifyContent(FlexAlign.Center)
      .backgroundColor($r('app.color.onboarding_background'))
      .onAreaChange((_: Area, area: Area) => { this.viewportWidth = Number(area.width); })
  }
}
