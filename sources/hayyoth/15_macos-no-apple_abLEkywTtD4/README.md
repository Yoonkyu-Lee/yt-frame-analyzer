# 해체 분석: "macOS - but better - No Apple"

- 영상: https://youtu.be/abLEkywTtD4 (6:06), 태그 `[noapple]`
- 결과물: 설정 과정 정리 (이 README), 전사 [transcripts/](transcripts/) (5구간)
- 방법: 프레임 366장 → 웹캠을 가린 키프레임 224장 → 5구간 전사(글 중심 형식). 오디오는 쓰지 않았습니다.

## 1. 한 일

| 시각 | 한 일 |
|---|---|
| 0:00~1:17 | 미리 적어 둔 노트에서 `defaults write -g` 두 줄을 복사해 실행합니다. 떠 있는 사이드바를 평평하게(`NSSplitViewItemSidebarDefaultsToFloatingAppearance -bool false`), 창 모서리 둥글기를 줄입니다(`NSConvolutionOverride1 -float 4`). Finder를 다시 띄워 확인합니다 |
| 1:17~2:29 | 해상도를 1280×800으로, 배경을 그러데이션 없는 단색으로 바꿉니다. Dock은 자동 숨김에 크기를 줄이고, 강조 색과 텍스트 강조색을 Graphite로, 아이콘을 Tinted로, 메뉴 막대를 항상 자동 숨김으로 둡니다 |
| 2:29~3:58 | 손쉬운 사용에서 대비 증가를 켭니다(투명도 줄이기도 함께 켜짐). Apple 계정에는 로그인하지 않습니다. 터미널 주석: `# Look good now`, `# How remove appple , macos without apple`, `# Apple track you`, `# remove it`. 외부로 나가는 연결을 막는 방화벽 **LuLu**(Objective-See의 무료 공개 소프트웨어)를 `brew install lulu`로 설치합니다 |
| 3:58~4:55 | LuLu 네트워크 확장을 켭니다. 기본 허용(Apple 프로그램, 설치된 프로그램, DNS)은 그대로 두고, Firefox, `curl`, `ping`이 연결할 때 뜨는 알림을 보여 줍니다 |
| 4:55~6:05 | 지도 서비스(`com.apple.geod` → Apple 지도 CDN)와 LuLu 자신의 업데이트 확인을 막고, **모든 프로그램에 대해 `*.apple.com`과 `*.icloud.com`을 차단**하는 규칙을 더합니다. 팁 앱은 "Content is Unavailable", 지도는 빈 격자만 보입니다. 메모: **"apple can't track you anymore"** → **"this is clean mac"** |

## 2. 해석

- **14번과 짝을 이루는 영상입니다.** 14번이 편집기에서 장식과 원격 측정을 걷어 냈다면, 15번은 운영체제에서 같은 일을 합니다. 화면을 평평하고 조용하게 만들고(둥근 모서리, 투명도, 색을 줄임), 제조사로 나가는 연결을 끊습니다.
- **"No Apple"은 기기를 버리는 것이 아니라 기기의 주인이 되는 것입니다.** 맥은 계속 쓰지만, Apple 서버와의 통신을 방화벽으로 직접 통제합니다. 12번의 결론("사람을 묶는 체계가 싫다")을 자기 컴퓨터에 적용한 예입니다.
- **도구는 공개 소프트웨어와 한 줄 명령입니다.** `defaults write`, `brew install`, LuLu 규칙 두 줄. 따로 만든 스크립트나 유료 도구는 없습니다.
- 다만 화면에서 확인된 차단은 `*.apple.com`, `*.icloud.com` 규칙과 지도 서비스 하나입니다. LuLu는 기본 설정에서 Apple 프로그램을 허용하므로, Apple 서비스가 실제로 얼마나 막혔는지는 영상만으로는 알 수 없습니다.
