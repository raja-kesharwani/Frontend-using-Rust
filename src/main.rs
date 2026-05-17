use yew::prelude::*;

struct Category {
    title: &'static str,
    subtitle: &'static str,
    image: &'static str,
}

#[function_component(Navbar)]
fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="logo">{ "LUXORIA" }</div>

            <ul class="nav-links">
                <li><a href="#">{ "Home" }</a></li>
                <li><a href="#categories">{ "Categories" }</a></li>
                <li><a href="#">{ "Collections" }</a></li>
                <li><a href="#">{ "About" }</a></li>
            </ul>

            <div class="nav-actions">
                <button class="icon-button">{ "🔍" }</button>
                <button class="icon-button">{ "🛒" }</button>
            </div>
        </nav>
    }
}

#[function_component(Hero)]
fn hero() -> Html {
    html! {
        <section class="hero">
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
}

#[function_component(CategoryCard)]
fn category_card(props: &CategoryCardProps) -> Html {
    html! {
        <article class="category-card">
            <img src={props.image} alt={props.title} />

            <div class="category-overlay">
                <p>{ props.subtitle }</p>
                <h3>{ props.title }</h3>
                <button>{ "Explore" }</button>
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
        },
        Category {
            title: "Men",
            subtitle: "Street ready",
            image: "assets/category-men.png",
        },
        Category {
            title: "Accessories",
            subtitle: "Glow with style",
            image: "assets/category-accessories.png",
        },
        Category {
            title: "New Arrivals",
            subtitle: "Latest collection",
            image: "assets/new-arrival.jpeg",
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
                            />
                        }
                    }).collect::<Html>()
                }
            </div>
        </section>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="page">
            <Navbar />
            <Hero />
            <Categories />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
