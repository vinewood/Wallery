/// Resolve the best image URL for a given target screen resolution.
pub fn resolve_best_url(url: &str, source: &str, target_w: u32, target_h: u32) -> String {
    match source {
        "bing" => resolve_bing_url(url, target_w, target_h),
        "unsplash" => resolve_unsplash_url(url, target_w, target_h),
        "pexels" => resolve_pexels_url(url, target_w, target_h),
        _ => url.to_string(), // wallhaven, nasa: pass through as-is
    }
}

/// Bing: URLs look like ".../OHR.xxx_1920x1080.jpg" or "..._UHD.jpg".
/// Replace resolution suffix with one matching screen size.
fn resolve_bing_url(url: &str, target_w: u32, target_h: u32) -> String {
    let max_dim = target_w.max(target_h);
    let suffix = if max_dim > 2500 {
        "_UHD.jpg"
    } else if max_dim > 1200 {
        "_1920x1080.jpg"
    } else {
        "_1366x768.jpg"
    };

    // Check if already has the desired suffix
    if url.ends_with(suffix) {
        return url.to_string();
    }

    // Try to replace known resolution patterns
    for res in ["_UHD", "_1920x1080", "_1366x768", "_640x360"] {
        if url.contains(res) {
            let new_res = suffix.trim_end_matches(".jpg");
            return url.replace(res, new_res);
        }
    }

    // Fallback: no recognizable pattern
    url.to_string()
}

/// Unsplash: `raw` URLs can take `&w=` and `&h=` parameters.
/// Regular/small/thumb URLs are left as-is.
fn resolve_unsplash_url(url: &str, target_w: u32, target_h: u32) -> String {
    let w = if target_w < 1920 { target_w } else { 1920 };
    let h = if target_h < 1080 { target_h } else { 1080 };

    if url.contains("&w=") || url.contains("?w=") {
        // Already has width param — leave as-is (regular/small/thumb URLs)
        url.to_string()
    } else {
        // It's a `raw` URL — append w and h params
        let separator = if url.contains('?') { "&" } else { "?" };
        format!("{}{}w={}&h={}&q=85&fm=jpg&fit=crop", url, separator, w, h)
    }
}

/// Pexels: URLs are already sized by the API; leave as-is.
fn resolve_pexels_url(url: &str, _target_w: u32, _target_h: u32) -> String {
    url.to_string()
}
