# Analytics Setup Guide

This guide explains how to set up analytics and conversion tracking for the PriceWhisperer website.

## Google Analytics 4 Setup

### 1. Create a Google Analytics 4 Property

1. Go to [Google Analytics](https://analytics.google.com/)
2. Sign in with your Google account
3. Click "Admin" (gear icon) in the bottom left
4. In the "Property" column, click "Create Property"
5. Enter property name: "PriceWhisperer"
6. Select reporting time zone and currency
7. Click "Next" and fill in business information
8. Click "Create" and accept terms
9. Get your Measurement ID (format: `G-XXXXXXXXXX`) from the "Data Streams" section

### 2. Configure Environment Variable

Create a `.env` file in the `website/` directory:

```bash
# Copy the example file
cp .env.example .env

# Or create manually
touch .env
```

Add your Measurement ID to the `.env` file:

```bash
VITE_GA_MEASUREMENT_ID=G-XXXXXXXXXX
```

Replace `G-XXXXXXXXXX` with your actual Google Analytics 4 Measurement ID.

**Important:** 
- Never commit `.env` to version control (it's in `.gitignore`)
- Add `.env.example` to version control with placeholder values
- In production, set this as an environment variable in your hosting platform

### 3. Events Being Tracked

The following events are automatically tracked:

#### Page Views
- **Event:** `page_view`
- **Triggered:** On every page load
- **Data:** `page_path`

#### Email Captures
- **Event:** `email_capture`
- **Category:** `lead_generation`
- **Label:** `hero_email_capture`
- **Triggered:** When user submits email in hero section

#### CTA Clicks
- **Event:** `cta_click`
- **Category:** `engagement`
- **Labels:**
  - `hero_start_trial`
  - `hero_watch_demo`
- **Triggered:** When user clicks primary CTAs

#### Pricing Interactions
- **Event:** `pricing_toggle`
- **Category:** `engagement`
- **Label:** `monthly` or `annual`
- **Triggered:** When user toggles billing period

#### Plan Selection
- **Event:** `plan_selected`
- **Category:** `conversion`
- **Label:** Plan name (Starter/Professional/Enterprise)
- **Value:** Plan price
- **Triggered:** When user clicks "Get Started" or "Start Free Trial"

#### FAQ Engagement
- **Event:** `faq_interaction`
- **Category:** `engagement`
- **Label:** FAQ item ID
- **Triggered:** When user expands/collapses FAQ items

## Analytics Utility Functions

A utility module is available at `src/utils/analytics.ts` with helper functions for tracking:

```typescript
import { trackEvent, trackPageView, trackEmailCapture, trackCTAClick } from './utils/analytics';

// Track custom events
trackEvent('button_click', {
  event_category: 'engagement',
  event_label: 'hero_cta',
  value: 99
});

// Track page views
trackPageView('/pricing', 'Pricing - PriceWhisperer');

// Track email captures
trackEmailCapture('hero');

// Track CTA clicks
trackCTAClick('hero_start_trial', 'hero');
```

### Available Utility Functions

- `trackEvent(eventName, params)` - Track any custom event
- `trackPageView(path, title?)` - Track page views
- `trackEmailCapture(source)` - Track email form submissions
- `trackCTAClick(label, location)` - Track CTA button clicks
- `trackPricingInteraction(action, label?, value?)` - Track pricing page interactions
- `trackFAQInteraction(faqId, action)` - Track FAQ expand/collapse
- `trackROICalculator(action, value?)` - Track ROI calculator interactions

## Custom Events

You can also add custom event tracking directly:

```typescript
if (typeof window !== 'undefined' && (window as any).gtag) {
  (window as any).gtag('event', 'event_name', {
    event_category: 'category_name',
    event_label: 'label_name',
    value: 123 // optional numeric value
  });
}
```

**Recommendation:** Use the utility functions from `analytics.ts` for consistency and easier maintenance.

## Conversion Goals Setup

In Google Analytics 4, set up the following conversion events:

1. **Email Capture** (`email_capture`)
2. **Trial Signup** (`plan_selected` with value > 0)
3. **Demo Request** (if you add a demo form)

### Setting Up Conversions in GA4

1. Go to Admin → Events
2. Mark events as conversions:
   - `email_capture`
   - `plan_selected`

## Additional Analytics Tools

### Hotjar (Optional)

For heatmaps and session recordings:

1. Sign up at [Hotjar](https://www.hotjar.com/)
2. Add the tracking code to `index.html`:

```html
<script>
  (function(h,o,t,j,a,r){
    h.hj=h.hj||function(){(h.hj.q=h.hj.q||[]).push(arguments)};
    h._hjSettings={hjid:YOUR_HOTJAR_ID,hjsv:6};
    a=o.getElementsByTagName('head')[0];
    r=o.createElement('script');r.async=1;
    r.src=t+h._hjSettings.hjid+j+h._hjSettings.hjsv;
    a.appendChild(r);
  })(window,document,'https://static.hotjar.com/c/hotjar-','.js?sv=');
</script>
```

### Facebook Pixel (Optional)

For retargeting campaigns:

1. Get your Pixel ID from Facebook Business Manager
2. Add to `App.tsx` in the `onMount` function:

```typescript
// Facebook Pixel
const fbPixelId = import.meta.env.VITE_FB_PIXEL_ID || '';
if (fbPixelId) {
  const script = document.createElement('script');
  script.innerHTML = `
    !function(f,b,e,v,n,t,s)
    {if(f.fbq)return;n=f.fbq=function(){n.callMethod?
    n.callMethod.apply(n,arguments):n.queue.push(arguments)};
    if(!f._fbq)f._fbq=n;n.push=n;n.loaded=!0;n.version='2.0';
    n.queue=[];t=b.createElement(e);t.async=!0;
    t.src=v;s=b.getElementsByTagName(e)[0];
    s.parentNode.insertBefore(t,s)}(window, document,'script',
    'https://connect.facebook.net/en_US/fbevents.js');
    fbq('init', '${fbPixelId}');
    fbq('track', 'PageView');
  `;
  document.head.appendChild(script);
}
```

## Testing Analytics

### Test in Development

1. Open browser DevTools → Network tab
2. Filter by "gtag" or "collect"
3. Interact with the site (click CTAs, submit forms)
4. Verify events are being sent

### Google Analytics DebugView

1. Install [Google Analytics Debugger](https://chrome.google.com/webstore/detail/google-analytics-debugger/jnkmfdileelhofjcijamephohjechhna) Chrome extension
2. Enable it
3. Open GA4 → Admin → DebugView
4. See real-time events as you interact with the site

## Key Metrics to Monitor

### Traffic Metrics
- Monthly unique visitors
- Traffic sources (organic, paid, direct, referral)
- Bounce rate
- Average session duration
- Pages per session

### Conversion Metrics
- Email capture rate
- Trial signup rate
- Trial → paid conversion rate
- Demo request rate
- Feature page engagement

### Engagement Metrics
- Time on site
- Scroll depth
- CTA click-through rate
- FAQ interaction rate
- Pricing toggle usage

## Troubleshooting

### Events Not Tracking

1. Check that `VITE_GA_MEASUREMENT_ID` is set correctly
2. Verify GA4 property is active
3. Check browser console for errors
4. Use GA DebugView to see if events are received

### Development vs Production

- Analytics only loads if `VITE_GA_MEASUREMENT_ID` is set
- In development, events will be logged to console
- In production, ensure environment variable is set in your deployment platform

## Privacy & GDPR

- Ensure you have a privacy policy
- Consider adding cookie consent banner
- Allow users to opt-out of tracking
- Document data collection in privacy policy

