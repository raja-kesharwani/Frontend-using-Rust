use yew::prelude::*;

struct Category {
    title: &'static str,
    subtitle: &'static str,
    image: &'static str,
    link: &'static str,
}

#[function_component(Navbar)]
fn navbar() -> Html {
    let menu_open = use_state(|| false);
    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };
    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(false))
    };

    html! {
        <nav class="navbar">
            <a class="logo" href="#home" onclick={close_menu.clone()}>{ "LUXORIA" }</a>

            <ul class={classes!("nav-links", (*menu_open).then_some("open"))}>
                <li><a href="#home" onclick={close_menu.clone()}>{ "Home" }</a></li>
                <li><a href="#categories" onclick={close_menu.clone()}>{ "Categories" }</a></li>
                <li><a href="#about" onclick={close_menu.clone()}>{ "About" }</a></li>
                <li><a href="#contact" onclick={close_menu.clone()}>{ "Contact" }</a></li>
            </ul>

            <div class="nav-actions">
                <button class="icon-button">{ "🔍" }</button>
                <button class="icon-button">{ "🛒" }</button>
                <button class="menu-button" onclick={toggle_menu} aria-label="Toggle menu">
                    <span></span>
                    <span></span>
                    <span></span>
                </button>
            </div>
        </nav>
    }
}

#[function_component(Hero)]
fn hero() -> Html {
    html! {
        <section class="hero" id="home">
            <div class="hero-content">
                <p class="hero-label">{ "New Collection 2026" }</p>

                <h1 class="hero-title">
                    { "Elevate Your Style With Modern Fashion" }
                </h1>

                <p class="hero-text">
                    { "Discover premium clothing designed for confidence, comfort, and timeless elegance. Explore our latest collection made for everyday luxury." }
                </p>

                <div class="hero-buttons">
                    <button class="primary-button">{ "Shop Now" }</button>
                    <button class="secondary-button">{ "Explore Collection" }</button>
                </div>
            </div>

            <div class="hero-image-box">
                <img src="assets/hero-model.png" alt="Luxoria fashion hero" />
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct CategoryCardProps {
    title: &'static str,
    subtitle: &'static str,
    image: &'static str,
    link: &'static str,
}

#[function_component(CategoryCard)]
fn category_card(props: &CategoryCardProps) -> Html {
    html! {
        <article class="category-card">
            <img src={props.image} alt={props.title} />

            <div class="category-overlay">
                <p>{ props.subtitle }</p>
                <h3>{ props.title }</h3>
                <a href={props.link}>{ "Explore" }</a>
            </div>
        </article>
    }
}

#[function_component(Categories)]
fn categories() -> Html {
    let categories = vec![
        Category {
            title: "Women",
            subtitle: "Fresh arrivals",
            image: "assets/category-women.png",
            link: "women.html",
        },
        Category {
            title: "Men",
            subtitle: "Street ready",
            image: "assets/category-men.png",
            link: "men.html",
        },
        Category {
            title: "Accessories",
            subtitle: "Glow with style",
            image: "assets/category-accessories.png",
            link: "accessories.html",
        },
        Category {
            title: "New Arrivals",
            subtitle: "Latest collection",
            image: "assets/new-arrival.jpeg",
            link: "new-arrivals.html",
        },
    ];

    html! {
        <section class="categories-section" id="categories">
            <div class="section-heading">
                <p>{ "Shop by Category" }</p>
                <h2>{ "Curated Fashion For Every Style" }</h2>
            </div>

            <div class="categories-grid">
                {
                    categories.iter().map(|category| {
                        html! {
                            <CategoryCard
                                title={category.title}
                                subtitle={category.subtitle}
                                image={category.image}
                                link={category.link}
                            />
                        }
                    }).collect::<Html>()
                }
            </div>
        </section>
    }
}

#[function_component(About)]
fn about() -> Html {
    html! {
        <section class="about-section" id="about">
            <div>
                <p>{ "About Luxoria" }</p>
                <h2>{ "Modern fashion for everyday confidence." }</h2>
            </div>
            <p>
                { "Luxoria brings together refined clothing and statement accessories for people who want their everyday wardrobe to feel polished, comfortable, and easy to style." }
            </p>
        </section>
    }
}

#[function_component(Contact)]
fn contact() -> Html {
    html! {
        <section class="contact-section" id="contact">
            <div class="section-heading">
                <p>{ "Contact" }</p>
                <h2>{ "Need help with your Luxoria order?" }</h2>
            </div>

            <div class="contact-grid">
                <article>
                    <h3>{ "Customer Care" }</h3>
                    <p>{ "support@luxoria.com" }</p>
                </article>
                <article>
                    <h3>{ "Shipping" }</h3>
                    <p>{ "Free delivery on orders above ₹4999." }</p>
                </article>
                <article>
                    <h3>{ "Returns" }</h3>
                    <p>{ "Easy returns within 7 days of delivery." }</p>
                </article>
            </div>
        </section>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="footer-brand">
                <h2>{ "LUXORIA" }</h2>
                <p>{ "Modern fashion for everyday confidence and luxury." }</p>
            </div>

            <div class="footer-links">
                <div>
                    <h4>{ "Shop" }</h4>
                    <a href="women.html">{ "Women" }</a>
                    <a href="men.html">{ "Men" }</a>
                    <a href="accessories.html">{ "Accessories" }</a>
                </div>

                <div>
                    <h4>{ "Company" }</h4>
                    <a href="#about">{ "About" }</a>
                    <a href="#contact">{ "Contact" }</a>
                </div>

                <div>
                    <h4>{ "Support" }</h4>
                    <a href="#contact">{ "Shipping" }</a>
                    <a href="#contact">{ "Returns" }</a>
                    <a href="#contact">{ "FAQs" }</a>
                </div>
            </div>

            <div class="footer-bottom">
                <p>{ "© 2026 Luxoria. All rights reserved." }</p>

            </div>
        </footer>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="page">
            <Navbar />
            <Hero />
            <Categories />
            <About />
            <Contact />
            <Footer />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
