<script lang="ts">
  import { t } from '../lib/i18n';
</script>

<!-- Hero -->
<div class="glass-card about-hero">
  <img src="/cube_icon.png" alt="Wallery Logo" class="about-logo-large" />
  <div class="about-appname">Wallery</div>
  <div class="about-version">{$t('app.version')}</div>
</div>

<!-- Story -->
<div class="glass-card about-story">
  <div class="section-header">
    <span class="section-title">{$t('about.story_title')}</span>
  </div>
  <div class="story-block">
    <p>{$t('about.story_p1')}</p>
    <br>
    <p>{$t('about.story_p2')}</p>
    <br>
    <p>{$t('about.story_p3')}</p>
    <br>
    <p style="font-weight: 500; color: var(--accent, #5b9cf5);">{$t('about.story_p4')}</p>
  </div>
</div>

<!-- Stats -->
<div class="glass-card">
  <div class="about-grid">
    <div class="about-stat">
      <div class="about-stat-value">100%</div>
      <div class="about-stat-label">{$t('about.free_badge')}</div>
    </div>
    <div class="about-stat">
      <div class="about-stat-value">MIT</div>
      <div class="about-stat-label">{$t('about.opensource_badge')}</div>
    </div>
    <div class="about-stat">
      <div class="about-stat-value">~6MB</div>
      <div class="about-stat-label">{$t('about.size_badge')}</div>
    </div>
    <div class="about-stat">
      <div class="about-stat-value">&lt;15MB</div>
      <div class="about-stat-label">{$t('about.memory_badge')}</div>
    </div>
  </div>
</div>

<!-- Sponsor / Alipay -->
<div class="glass-card sponsor-card">
  <div class="sponsor-inner">
    <div class="sponsor-text">
      <div class="sponsor-title">☕ {$t('about.sponsor_title')}</div>
      <div class="sponsor-sub">{$t('about.sponsor_sub')}</div>
    </div>
    <div class="alipay-box">
      <img src="/alipay.jpg" alt="支付宝收款码" class="alipay-qr" />
      <div class="alipay-label">请用支付宝扫一扫支持作者</div>
    </div>
  </div>
</div>

<!-- Links -->
<div class="about-links">
  <a href="https://gitee.com/moshangjianjia/wallery" target="_blank" rel="noopener" class="about-link">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
    Gitee · Wallery 幕间
  </a>
</div>

<style>
  .about-hero {
    text-align: center;
    padding: 32px 24px;
    margin-bottom: 20px;
  }

  .about-logo-large {
    width: 96px;
    height: 96px;
    border-radius: 24px;
    object-fit: contain;
    filter: drop-shadow(0 8px 32px rgba(0,0,0,0.4)) drop-shadow(0 0 60px var(--accent-glow, rgba(91,156,245,0.25)));
    margin-bottom: 12px;
  }

  .about-appname {
    font-size: 32px;
    font-weight: 300;
    letter-spacing: -0.5px;
    background: linear-gradient(135deg, #e8eaed 0%, #5b9cf5 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 4px;
  }

  .about-version {
    font-size: 12px;
    color: var(--text-muted, #5f6672);
    font-weight: 400;
  }

  .about-story {
    margin-bottom: 20px;
  }

  .story-block {
    padding: 20px;
    border-radius: 12px;
    background: rgba(0,0,0,0.1);
    border-left: 3px solid var(--accent, #5b9cf5);
  }

  .story-block p {
    font-size: 13.5px;
    line-height: 1.8;
    color: var(--text-secondary, #9aa0ab);
  }

  .about-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .about-stat {
    text-align: center;
    padding: 20px 16px;
    border-radius: 12px;
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.06);
  }

  .about-stat-value {
    font-size: 28px;
    font-weight: 300;
    color: var(--accent, #5b9cf5);
  }

  .about-stat-label {
    font-size: 11.5px;
    color: var(--text-muted, #5f6672);
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin-top: 4px;
  }

  .sponsor-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary, #e8eaed);
    margin-bottom: 8px;
  }

  .sponsor-sub {
    font-size: 12px;
    color: var(--text-muted, #5f6672);
    margin-bottom: 16px;
    line-height: 1.6;
    white-space: pre-line;
  }

  .sponsor-card { margin-top: 20px; }
  .sponsor-inner { display: flex; align-items: center; gap: 24px; }
  .sponsor-text { flex: 1; }
  .alipay-box { text-align: center; flex-shrink: 0; }
  .alipay-qr { width: 100px; height: 100px; border-radius: 12px; object-fit: contain; }
  .alipay-label { font-size: 10px; color: var(--text-muted); margin-top: 6px;   }

  .about-links {
    display: flex;
    justify-content: center;
    gap: 12px;
    margin-top: 20px;
  }

  .about-link {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 10px 20px;
    border-radius: 100px;
    border: 1px solid rgba(255,255,255,0.10);
    background: rgba(255,255,255,0.04);
    color: var(--text-secondary, #9aa0ab);
    font-size: 12.5px;
    font-weight: 500;
    text-decoration: none;
    transition: all 0.3s;
    font-family: inherit;
  }

  .about-link:hover {
    border-color: var(--accent, #5b9cf5);
    color: var(--accent, #5b9cf5);
    background: var(--accent-soft, rgba(91,156,245,0.15));
  }
</style>
