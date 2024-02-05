use next_rs::prelude::*;
use next_rs::Link;

#[func]
pub fn LandingPage() -> Html {
    rsx! {
        <div class="min-h-screen flex flex-col items-center justify-center bg-gray-900 text-white">
            <div class="max-w-4xl mx-auto p-8 text-center">
                <header id="header">
                    <h1 class="text-5xl font-extrabold mb-8">{"Discover Your Journey"}</h1>
                </header>

                <nav class="text-blue-400 flex items-center justify-center text-blue space-x-6 mb-12">
                    <Link to="#home" class="text-lg hover:underline">{"Home"}</Link>
                    <Link to="#about" class="text-lg hover:underline">{"About"}</Link>
                    <Link to="/#features" scroll=true scroll_behavior="smooth" class="text-lg hover:underline">{"Features"}</Link>
                    <Link to="#contact" scroll=true scroll_behavior="instant" scroll_offset=10.0 class="text-lg hover:underline">{"Contact"}</Link>
                    <Link to="#portfolio" aria_current="page" class="text-lg hover:underline">{"Portfolio"}</Link>
                    <Link to="#services" class="text-lg hover:underline">{"Services"}</Link>
                    <Link to="#not-found" scroll_offset=300.0 scroll=true scroll_behavior="smooth" class="text-lg hover:underline">{"Not Found"}</Link>
                </nav>

                <section id="home" class="mb-12">
                    <h2 class="text-3xl font-semibold mb-4">{"Unleash Creativity and Innovation"}</h2>
                    <div class="flex flex-col items-center justify-center bg-gray-700 p-8 rounded-md shadow-lg">
                        <h3 class="text-2xl font-semibold mb-4">{"Explore Our Innovative Solutions"}</h3>
                        <p class="text-gray-400">
                            {"At Next RS, we strive to provide cutting-edge solutions tailored to meet your unique needs. Our team of experts is dedicated to pushing boundaries and delivering excellence. Whether it's designing captivating user experiences or developing robust backend systems, we're here to transform ideas into reality."}
                        </p>
                        <ul class="mt-4 text-gray-300">
                            <li>{"Customized web and mobile applications"}</li>
                            <li>{"State-of-the-art design and development"}</li>
                            <li>{"Agile project management for efficient delivery"}</li>
                            <li>{"Continuous innovation and technology integration"}</li>
                        </ul>
                        <p class="mt-4 text-blue-400 hover:underline cursor-pointer">
                            {"Learn more about our services"}
                        </p>
                    </div>
                </section>

                <section id="about" class="mb-12">
                    <h2 class="text-3xl font-semibold mb-4">{"Have Questions? We've Got Answers"}</h2>
                    <div id="faq-description" class="hidden">{"Explore our comprehensive FAQ section for more information."}</div>
                    <Link to="/faq" aria_describedby="faq-description" aria_hidden="true" class="text-blue-400 text-lg hover:underline">{"Read FAQs"}</Link>
                </section>

                <section id="features" class="mb-12">
                    <h2 class="text-3xl font-semibold mb-4">{"Dive Into Our Insights"}</h2>
                    <Link to="/blog" aria_expanded="true" class="text-lg hover:underline text-blue-400">{"Read Blog"}</Link>
                </section>

                <section id="contact" class="mb-12">
                    <h2 class="text-3xl font-semibold mb-4">{"Stay Updated with Exciting News"}</h2>
                    <Link scroll=true scroll_behavior="smooth" scroll_offset=10.0 to="/subscribe" aria_pressed="true" class="text-blue-400 text-lg hover:underline">{"Subscribe Now"}</Link>
                </section>

                <section id="portfolio" class="mb-12">
                    <h2 class="text-3xl font-semibold mb-4">{"Experience Through Video"}</h2>
                    <Link to="#header" scroll=true scroll_behavior="smooth" aria_controls="video-player" scroll_offset=10.0 class="text-blue-400 text-lg hover:underline">{"Watch Video"}</Link>
                </section>

                <section id="services" class="mb-12">
                    <h2 class="text-3xl font-semibold mb-4">{"Our Services"}</h2>
                    {"Service content."}
                </section>

                <section id="team" class="mb-12">
                    <h2 class="text-3xl font-semibold mb-4">{"Meet Our Team"}</h2>
                    {"Team members."}
                </section>

                <footer class="mt-16">
                    <p class="text-gray-500">{"© 2024 Next RS. All rights reserved."}</p>
                </footer>
            </div>
        </div>
    }
}
