# 해체 분석: "Pure HTML just works and beats AI Slop"

- 영상: https://youtu.be/fG7mM6Yymaw (10:01), 태그 `[html]`
- 결과물: 논증 정리와 손으로 쓴 `index.html`(보이는 부분) (이 README), 전사 [transcripts/](transcripts/) (7구간)
- 방법: 프레임 → 웹캠을 가린 키프레임 312장 → 7구간 전사. 오디오는 쓰지 않았습니다. 메일함, Telegram, 브라우저 기록이 화면에 비쳤지만 전사에서는 제외하거나 가렸습니다.

## 1. 논제

처음부터 끝까지 붙어 있는 하늘색 메모입니다:
> **"Your website is your soul, it must reflect your personality."**
> **"Content and accessibility are key; keep your website extremely simple and ensure it performs its intended function."**

## 2. 흐름

| 시각 | 화면 | 노란 메모 |
|---|---|---|
| 0:00~3:05 | 요즘 사이트들(AI 에이전트 제품, 앱 모음, GitHub 등)을 훑고, 스프레드시트에 **"Websites slop by AI"** 다섯 개와 **"Legend"** 약 23개(손으로 만든 개인 사이트)를 나란히 적음 | "what website look nowday" → "why all look same" → "same , slop by AI" |
| 3:05~4:10 | AI가 만든 포트폴리오 예시를 "slop"으로 보여 주고, "Legend" 사이트들을 차례로 엶: bellard.org, doc.cat-v.org, nullprogram.com, zserge.com, paulgraham.com, templeos.org | "trash , nothing special" → "this is good website , legend" |
| 4:10~6:00 | 자기 사이트 hayyoth.org: 스타일 없는 HTML. 사람들이 보내는 메일, 검색 결과, Cloudflare 통계(요청 9.35k, 방문 4.16k) | 개인 사이트는 서로 달라야 하고 단순해야 한다, 사람들은 내용만 본다 |
| 6:00~8:36 | "웹 앱이라면?"에 대한 답: DuckDuckGo의 HTML 전용 검색, Telegram Web의 두 열 화면. Codeberg 저장소 페이지는 JS를 끄면 안 보인다는 것을 개발자 도구로 시연 | "html work", "very simple. just two column, that all", "No AI popup" |
| 8:36~9:59 | 보너스: 자기 사이트 만드는 법. 10달러짜리 도메인을 사고, `index.html`을 vim으로 쓰고(`"index.html" 26L, 1142B`), Cloudflare Workers에 정적 파일로 올려 배포 | "bonus: how i build my website" → "buy 10$ claudflare domain" → "write html" → **"Boom / all done / that it , simple , clean, work"** |

## 3. 손으로 쓴 `index.html` (8:59~9:01, 26줄 중 보이는 부분)

창 오른쪽 끝에서 잘린 부분은 `…`로 표시했습니다. 22~25줄은 보이지 않았지만, 렌더된 화면으로 보아 바닥글 링크("Codeberg", "Email")와 닫는 태그입니다.

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial…
  <title>Hayyoth</title>
</head>
<body>
  <h1 style="margin: 0;">Hayyoth</h1>
  <p>The guardian of the craft of manual programming.</p>
  <ul>
    <li><a href="https://www.youtube.com/@hayyoth-org/posts"…   (My thoughts)
    <li><a href="https://youtube.com/playlist?list=…            (Opinions on AI)
    <li><a href="https://youtube.com/playlist?list=…            (Build your own X from scratch)
    <li><a href="https://youtube.com/playlist?list=…            (Learn C)
    <li><a href="https://youtube.com/playlist?list=…            (Data Structures in C)
    <li><a href="https://youtube.com/playlist?list=…            (Lisp Language)
  </ul>
  <hr>
  <footer>
    <a href="https://www.youtube.com/@hayyoth-org">Youtube</…
    … (Codeberg, Email)
</html>
```

CSS는 `<h1>`의 `margin: 0` 한 줄이 전부입니다. 브라우저 기본 스타일로 렌더됩니다.

## 4. 해석

- **"AI slop"은 코드보다 디자인에서 먼저 보인다는 주장입니다.** AI로 만든 사이트들은 서로 닮았고("why all look same"), 사람이 만든 오래된 개인 사이트는 각자 다릅니다. 사이트는 그 사람을 드러내야 한다는 하늘색 메모가 기준입니다.
- **필요한 만큼만 만듭니다.** 개인 사이트에는 HTML 26줄, CSS 한 줄, 정적 호스팅이면 충분하다는 것을 직접 보여 줍니다. 웹 앱도 두 열이면 된다는 예(DuckDuckGo HTML, Telegram Web)를 들고, JS가 없으면 보이지 않는 페이지를 반례로 보여 줍니다.
- **증거는 결과로 제시합니다.** 단순한 사이트로도 메일과 방문(4천여 건)이 온다는 것을 통계로 보여 줍니다. 10번의 "단순한 것이 이긴다"를 웹에 적용한 영상입니다.
