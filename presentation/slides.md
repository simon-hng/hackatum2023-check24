---
theme: seriph
background: https://images.pexels.com/photos/2235924/pexels-photo-2235924.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2
class: text-center
highlighter: shiki
lineNumbers: false
info: |
  ## Slidev Starter Template
  Presentation slides for developers.

  Learn more at [Sli.dev](https://sli.dev)
drawings:
  persist: false
transition: slide-left
title: Welcome to Slidev
mdc: true
---

# Red Crab

Redis + Rust = 🔥🚀

<div class="pt-12">
  <span @click="$slidev.nav.next" class="px-2 py-1 rounded cursor-pointer" hover="bg-white bg-opacity-10">
    Press Space for next page <carbon:arrow-right class="inline"/>
  </span>
</div>

<div class="abs-br m-6 flex gap-2">
  <button @click="$slidev.nav.openInEditor()" title="Open in Editor" class="text-xl slidev-icon-btn opacity-50 !border-none !hover:text-white">
    <carbon:edit />
  </button>
  <a href="https://github.com/slidevjs/slidev" target="_blank" alt="GitHub" title="Open in GitHub"
    class="text-xl slidev-icon-btn opacity-50 !border-none !hover:text-white">
    <carbon-logo-github />
  </a>
</div>

<!--
The last comment block of each slide will be treated as slide notes. It will be visible and editable in Presenter Mode along with the slide. [Read more in the docs](https://sli.dev/guide/syntax.html#notes)
-->

---
transition: fade-out
layout: quote
---

# What is Redis?
- The open source, in-memory data store used by millions of developers as a database, cache, streaming engine, and message broker.

---
transition: fade-out
---

# Redis GEORADIUS - Finding locations close to each other

> O(N+log(M)) where N is the number of elements inside the bounding box of the circular area delimited by center and radius and M is the number of items inside the index.

## Implementation in rust
```rust {all|2|3|4-8|all}
close_craftsmen_ids = state
    .connection_manager
    .geo_radius_by_member(
        "locations".to_string(),
        format!("postal:{}", postalcode),
        radius,
        Unit::Kilometers,
        RadiusOptions::default(),
```

---
transition: fade-out
---

# Starting radius for searching

```rust {all|1-4|9-14}
let mut radius = match extension_group {
    PostcodeExtensionDistanceGroup::GroupA => { 10.0 }
    PostcodeExtensionDistanceGroup::GroupB => { 20.0 }
    PostcodeExtensionDistanceGroup::GroupC => { 40.0 }

    let mut close_craftsmen_ids: Vec<String> = vec![];

    while close_craftsmen_ids.len() < required_craftsmen as usize {
        close_craftsmen_ids = state

        ... Getting ids with redis

        radius *= 2.0;
    }

};

```

This breaks if someone doens't have 20 craftsmen in their area

---
transition: fade-out
---

# Why does this work?

We save some data beforehand so that during runtime we can be faster

- location -> craftsmen
- location -> postal code

```rust{all|3|12}
connection_manager
    .geo_add::<String, (f64, f64, String), String>(
        "locations".to_string(),
        (postal.lon, postal.lat, key),
    )
    .await;

    ...

connection_manager
    .geo_add::<String, (f64, f64, String), String>(
        "locations".to_string(),
        (
            profile.service_provider_profile.lon,
            profile.service_provider_profile.lat,
            key,
        ),
    )
    .await;
```

---
transition: fade-out
---

# Why does this work?

We save some data together to reduce access to the db.

```rust{all|3|5|all}
pub struct Craftsman {
    #[serde(flatten)]
    pub quality_factors: QualityFactors,
    #[serde(flatten)]
    pub service_provider_profile: ServiceProviderProfiles,
    pub rank: Option<f64>,
    pub distance: Option<f64>,
}
```

```rust {all|6-7|all}
let craftsmen: Vec<entity::Craftsman> = service_provider_profiles
    .into_iter()
    .zip(quality_factors.into_iter())
    .map(
        |(service_provider_profile, quality_factors)| entity::Craftsman {
            quality_factors,
            service_provider_profile,
            rank: None,
            distance: None,
        },
    )
    .collect();
```

---
transition: fade-out
---

# 🦀

- In-memory database
- Usage of functions provided by the database
- Low-level language with zero-cost abstractions

= Speed?🚀


I guess we will see \:D
