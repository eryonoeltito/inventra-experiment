use axum::{Router, response::Html, routing::get};

const PAGE: &str = r##"<!doctype html>
<html lang="id">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="theme-color" content="#f4f7f9">
  <title>INVENTRA — Inventaris yang tertata</title>
  <style>
    :root { color-scheme: light; --ink: #172d43; --blue: #235b83; --blue-soft: #dceaf2; --paper: #f4f7f9; --white: #fff; --line: #d7e1e7; --muted: #5e7181; --green: #43826e; font-family: "Avenir Next", Avenir, "Segoe UI", sans-serif; }
    * { box-sizing: border-box; }
    body { margin: 0; min-width: 320px; background: var(--paper); color: var(--ink); }
    .page { width: min(1180px, calc(100% - 64px)); margin: 0 auto; }
    header { height: 88px; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid var(--line); }
    .brand { display: flex; align-items: center; gap: 12px; color: var(--ink); text-decoration: none; }
    .brand:focus-visible { outline: 3px solid var(--green); outline-offset: 5px; border-radius: 3px; }
    .brand-mark { width: 34px; height: 34px; display: grid; place-items: center; background: var(--blue); color: var(--white); border-radius: 9px 9px 9px 3px; font-size: 17px; font-weight: 700; }
    .brand-name { font-size: 17px; font-weight: 750; letter-spacing: .045em; }
    .brand-subtitle { display: block; color: var(--muted); font-size: 10px; letter-spacing: .09em; margin-top: 2px; }
    .header-note { color: var(--muted); font-size: 13px; }
    main { min-height: calc(100svh - 114px); display: grid; grid-template-columns: .9fr 1.1fr; align-items: center; gap: clamp(36px, 7vw, 92px); padding: 62px 0 78px; }
    .intro { max-width: 470px; }
    h1 { max-width: 11ch; margin: 0; font-size: clamp(42px, 5.2vw, 68px); line-height: 1.04; letter-spacing: -.055em; font-weight: 680; }
    .intro p { max-width: 42ch; margin: 24px 0 0; color: var(--muted); font-size: 17px; line-height: 1.7; }
    .rule { width: 54px; height: 3px; margin-top: 34px; background: var(--green); }
    .art { position: relative; min-height: 430px; display: grid; place-items: center; overflow: hidden; background: var(--blue-soft); border-radius: 3px 44px 3px 3px; }
    .art::before, .art::after { content: ""; position: absolute; border: 1px solid rgba(35, 91, 131, .13); border-radius: 50%; }
    .art::before { width: 390px; height: 390px; }
    .art::after { width: 500px; height: 500px; }
    .rack { position: relative; z-index: 1; width: min(76%, 390px); height: 286px; padding: 0 18px; border: 8px solid var(--blue); border-top-width: 10px; border-bottom-width: 12px; display: flex; flex-direction: column; justify-content: space-between; background: rgba(255,255,255,.28); }
    .shelf { position: relative; height: 50%; border-bottom: 7px solid var(--blue); display: flex; align-items: end; gap: 10px; padding: 0 6px 7px; }
    .shelf:last-child { border-bottom: 0; }
    .box { display: block; height: var(--h); flex: 1; background: var(--white); border: 1px solid rgba(35,91,131,.22); }
    .box:nth-child(2n) { background: #c6dbe6; }
    .box:nth-child(3n) { border-top: 5px solid var(--green); }
    .box.small { flex: .7; }
    .caption { position: absolute; z-index: 2; right: 26px; bottom: 23px; color: var(--blue); font-size: 12px; letter-spacing: .025em; }
    footer { padding: 0 0 26px; color: #718391; font-size: 12px; }
    @media (max-width: 740px) {
      .page { width: min(100% - 40px, 560px); } header { height: 72px; } .header-note { font-size: 12px; }
      main { min-height: 0; grid-template-columns: 1fr; gap: 38px; padding: 56px 0 48px; }
      h1 { max-width: 12ch; font-size: clamp(42px, 11vw, 58px); } .intro p { font-size: 16px; }
      .art { min-height: 330px; } .rack { height: 230px; } .art::before { width: 300px; height: 300px; } .art::after { width: 390px; height: 390px; }
    }
    @media (max-width: 400px) { .page { width: calc(100% - 32px); } .header-note { max-width: 116px; text-align: right; line-height: 1.35; } .art { min-height: 285px; } .rack { width: 82%; height: 205px; } }
    @media (prefers-reduced-motion: reduce) { *, *::before, *::after { scroll-behavior: auto !important; } }
  </style>
</head>
<body>
  <div class="page">
    <header>
        <a class="brand" href="/" aria-label="INVENTRA, halaman awal"><span class="brand-mark" aria-hidden="true">I</span><span><span class="brand-name">INVENTRA</span><span class="brand-subtitle">Manajemen inventaris</span></span></a>
      <span class="header-note">Ruang kerja inventaris Anda</span>
    </header>
    <main>
      <section class="intro" aria-labelledby="page-title"><h1 id="page-title">Barang tertata, kerja lebih terarah.</h1><p>INVENTRA membantu pengelolaan barang dan gudang tetap jelas, rapi, dan mudah ditelusuri.</p><div class="rule" aria-hidden="true"></div></section>
      <div class="art" role="img" aria-label="Ilustrasi rak gudang dengan kotak barang yang tersusun"><div class="rack" aria-hidden="true"><div class="shelf"><span class="box" style="--h:52%"></span><span class="box small" style="--h:74%"></span><span class="box" style="--h:62%"></span><span class="box small" style="--h:46%"></span></div><div class="shelf"><span class="box small" style="--h:58%"></span><span class="box" style="--h:72%"></span><span class="box small" style="--h:45%"></span><span class="box" style="--h:64%"></span></div></div><span class="caption">Setiap barang punya tempat.</span></div>
    </main>
    <footer>INVENTRA · Manajemen inventaris barang dan gudang</footer>
  </div>
</body>
</html>"##;

async fn index() -> Html<&'static str> {
    Html(PAGE)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to 127.0.0.1:3000");
    println!("Frontend INVENTRA berjalan di http://127.0.0.1:3000");
    axum::serve(listener, app)
        .await
        .expect("frontend server failed");
}
