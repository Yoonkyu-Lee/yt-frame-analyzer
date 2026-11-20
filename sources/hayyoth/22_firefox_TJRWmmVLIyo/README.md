# 해체 분석: "firefox - but better"

- 영상: https://youtu.be/TJRWmmVLIyo (3:46), 태그 `[firefox]`
- 결과물: 설정 과정과 `userChrome.css` 정리 (이 README), 전사 [transcripts/](transcripts/) (3구간)
- 방법: 프레임 226장 → 웹캠을 가린 키프레임 152장 → 3구간 전사(글 중심 형식). 오디오는 쓰지 않았습니다. 프로필 경로의 계정 이름은 가렸습니다.

## 1. 한 일

화면에 설명하는 메모가 없습니다. 14번(VSCodium), 15번(macOS)처럼 무엇을 끄는지가 메시지입니다.

| 시각 | 한 일 |
|---|---|
| 0:00~0:20 | 설정 > **AI controls**: "Block AI enhancements"를 켜고 확인 창에서 Block(번역, 음성 인식, PDF 대체 텍스트, 탭 그룹 제안, 링크 미리보기 요약, Smart Window, 사이드바 챗봇이 막힘) |
| 0:20~0:30 | 개인 정보 > Firefox 데이터 수집: 기술·상호작용 데이터 전송, 기능 개선, 일일 사용 핑을 모두 끔 |
| 0:30~1:12 | 새 창·새 탭을 **빈 페이지**로, 북마크 도구 모음 "Never Show", Customize Toolbar에서 버튼을 거의 다 치움. `about:profiles`로 프로필 폴더 확인 |
| 1:12~2:25 | `about:config`에서 `toolkit.legacyUserProfileCustomizations.stylesheets` = `true`. 프로필 폴더에 `mkdir chrome`, vim으로 `userChrome.css`. 개발자 도구 설정에서 "browser chrome and add-on debugging toolboxes"를 켜고 **Browser Toolbox**의 요소 선택기로 Firefox 자체 화면을 검사 |
| 2:25~3:45 | Rules 창에서 `#TabsToolbar { display: none; }`을 시험한 뒤 파일로 옮김. 뒤로·앞으로 버튼도 추가. 다시 실행하니 탭 막대와 뒤로·앞으로 버튼이 사라짐. 탭은 "List all tabs" 목록으로 오감 |

최종 `userChrome.css` (vim: `"userChrome.css" 3L, 65B written`):
```css
#TabsToolbar, #back-button, #forward-button {
  display: none;
}
```

## 2. 해석

- **14·15번의 브라우저판입니다.** 편집기(14), 운영체제(15)에 이어 브라우저도 장식, 원격 측정, 그리고 **AI 기능**을 끕니다. AI 차단이 첫 동작입니다.
- **확장 기능 대신 텍스트 파일 하나로 도구를 바꿉니다.** 브라우저 화면도 CSS로 그려진다는 것을 이용합니다. Browser Toolbox로 이름(id)을 찾고, 개발자 도구에서 먼저 시험하고, vim으로 파일에 옮기는 순서는 18번의 "devtools에서 맞춘 뒤 소스로 옮긴다"와 같습니다.
- **탭 막대까지 없앱니다.** 탭은 목록으로만 고르고 이동은 주소창으로 합니다. 24번(w3m)의 텍스트 브라우저로 가는 중간 단계로 볼 수 있습니다.
