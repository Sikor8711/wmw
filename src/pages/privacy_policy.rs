use leptos::prelude::*;
#[component]
pub fn PrivacyPolicyPage() -> impl IntoView {
    view! {
    <div class="px-6 py-12">
        <h1 class="text-4xl font-bold text-gray-900 mb-2">"Privacy Policy"</h1>
        <p class="text-sm text-gray-400 mb-10 italic">"Last updated: 22 March 2026"</p>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"Who We Are"</h2>
        <p class="mb-4">"Wildly Magnetic is operated by Luiza Sikorska as a sole trader. In this policy, \"we\", \"us\" and
            \"our\" refers to Wildly Magnetic. You can contact us at: "
            <a class="text-amber-600 hover:underline" href="mailto:hello@wildlymagnetic.com">"hello@wildlymagnetic.com"</a>
        </p>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"What Data We Collect"</h2>
        <p class="mb-4">"We collect the following personal data:"</p>
        <ul class="list-disc list-inside space-y-2 mb-4 pl-4">
            <li>"Name and email address — when you sign up to our newsletter or waitlist"</li>
            <li>"Name, email address, and purchase details — when you buy a digital product through our site"</li>
            <li>"Name, email address, and message content — when you submit our contact form"</li>
        </ul>
        <p class="mb-4">"We also collect anonymous, aggregated analytics data (page views, referral sources, time on page)
        via Plausible Analytics. Plausible does not use cookies, does not track you across sites, and does not collect
        any personally identifiable information."</p>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"How We Use Your Data"</h2>
        <ul class="list-disc list-inside space-y-2 mb-4 pl-4">
            <li>"To send you emails you have opted into (newsletter, waitlist updates)"</li>
            <li>"To process and fulfil your purchase and provide customer support"</li>
            <li>"To respond to enquiries submitted via our contact form"</li>
            <li>"To understand how our website is used so we can improve it (Plausible analytics)"</li>
        </ul>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"Legal Basis for Processing"</h2>
        <ul class="list-disc list-inside space-y-2 mb-4 pl-4">
            <li>"Consent — for newsletter and waitlist emails. You can withdraw consent at any time by clicking unsubscribe
                in any email."</li>
            <li>"Contract — to fulfil your purchase."</li>
            <li>"Legitimate interests — to respond to contact form enquiries and to maintain basic anonymous analytics."
            </li>
        </ul>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"Payment Processing"</h2>
        <p class="mb-4">"Payments are processed by Paddle (paddle.com), who act as Merchant of Record. Paddle handle all
        payment card data directly — we never see or store your card details. Paddle's own privacy policy applies to
        payment data."</p>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"Email Marketing"</h2>
        <p class="mb-4">"We use Mautic (self-hosted) to manage and send emails. Your email address and name are stored on
        our secure server. We do not use Mautic's JavaScript tracker on our website. You can unsubscribe from any email
        at any time."</p>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"Cookies"</h2>
        <p class="mb-4">"This website uses one strictly necessary cookie to maintain your session if you are logged in. This
        cookie contains no personal information and is deleted when you close your browser. No consent is required for
        this cookie as it is essential for the site to function."</p>
        <p class="mb-4">"We do not use any advertising, tracking, or analytics cookies."</p>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"Data Sharing"</h2>
        <p class="mb-4">"We do not sell your data. We share your data only with:"</p>
        <ul class="list-disc list-inside space-y-2 mb-4 pl-4">
            <li>"Paddle — for payment processing"</li>
            <li>"Our hosting provider — your data is stored on servers located in the EU"</li>
        </ul>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"How Long We Keep Your Data"</h2>
        <ul class="list-disc list-inside space-y-2 mb-4 pl-4">
            <li>"Newsletter/waitlist subscribers — until you unsubscribe"</li>
            <li>"Purchase records — 7 years (legal requirement for financial records)"</li>
            <li>"Contact form submissions — up to 12 months"</li>
        </ul>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"Your Rights"</h2>
        <p class="mb-4">"Under UK GDPR you have the right to:"</p>
        <ul class="list-disc list-inside space-y-2 mb-4 pl-4">
            <li>"Access the personal data we hold about you"</li>
            <li>"Request correction of inaccurate data"</li>
            <li>"Request deletion of your data"</li>
            <li>"Withdraw consent at any time (for email marketing)"</li>
            <li>"Lodge a complaint with the ICO (ico.org.uk)"</li>
        </ul>
        <p class="mb-4">"To exercise any of these rights, email us at "
            <a class="text-amber-600 hover:underline" href="mailto:hello@wildlymagnetic.com">"hello@wildlymagnetic.com"</a>
        </p>

        <h2 class="text-xl font-bold text-gray-900 mt-10 mb-3">"Changes to This Policy"</h2>
        <p class="mb-4">"We may update this policy from time to time. The date at the top of this page will always reflect
            the latest version."</p>
    </div>
    }
}
