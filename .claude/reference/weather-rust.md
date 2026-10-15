# Referencia: script en Rust para comparar el tiempo de ayer con el de ahora

> Investigación del 15 de octubre de 2026. Los planes gratuitos de las APIs cambian con frecuencia, así que verifica los límites antes de depender de ellos.

---

## 1. APIs meteorológicas gratuitas (ranking)

| # | API | API key | Ayer + ahora en una llamada | Límite gratuito | Notas |
|---|-----|---------|-----------------------------|-----------------|-------|
| 1 | **Open-Meteo** | No | Sí (`current` + `past_days=1`) | Sin cuota fija para uso no comercial | Atribución obligatoria (CC BY 4.0) |
| 2 | **Visual Crossing** | Sí | Sí (Timeline API) | ~1.000 registros/día | `include=hours` consume más registros |
| 3 | **WeatherAPI.com** | Sí | No, son dos endpoints (`current` + `history`) | Entre 100K y 1M llamadas/mes según la fuente | 1 día de histórico en el plan gratuito |
| 4 | **AEMET OpenData** | Sí (gratis) | Datos horarios de las últimas 24 h | Límite estricto por minuto (HTTP 429) | Solo España, va por estación; caídas frecuentes |
| 5 | **OpenWeatherMap** | Sí | No | 1.000 llamadas/día | Su plan gratuito no incluye histórico, así que **no sirve** para este caso |
| – | Google Weather API | Sí + facturación | 24 h de histórico en caché | — | Excesivo para un script |

### Open-Meteo: petición de ejemplo

```
https://api.open-meteo.com/v1/forecast?latitude=LAT&longitude=LON&current=temperature_2m,relative_humidity_2m,precipitation,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,precipitation,wind_speed_10m&past_days=1&forecast_days=1&timezone=auto
```

- `current` da el estado actual.
- `hourly` con `past_days=1` da las horas de ayer. Busca la misma hora que `current.time` restándole 24 h.
- `timezone=auto` hace que las horas vengan en hora local.
- Para más historia, usa el endpoint archive: `https://archive-api.open-meteo.com/v1/archive` (ERA5, desde 1940).
- Atribución: «Weather data by Open-Meteo.com».

### WeatherAPI.com: endpoints

```
https://api.weatherapi.com/v1/current.json?key=KEY&q=CIUDAD
https://api.weatherapi.com/v1/history.json?key=KEY&q=CIUDAD&dt=YYYY-MM-DD
```

---

## 2. Crates de Rust

| # | Crate | Para qué |
|---|-------|----------|
| 1 | `openmeteo-rs` (v1) | Cliente tipado y asíncrono de Open-Meteo (forecast, archive, geocoding…). Valida los parámetros antes de enviar la petición |
| 2 | `serde` + `serde_json` | Deserializar el JSON si haces tu propio cliente |
| 3 | `reqwest` (feature `json`) + `tokio` | HTTP asíncrono estándar |
| 3b | `ureq` | HTTP síncrono y ligero; evita tokio en un script simple |
| 4 | `jiff` o `chrono` | Calcular «ayer a esta hora» y manejar zonas horarias (`jiff` es más moderno) |
| 5 | `anyhow` | Manejo de errores en binarios |
| 6 | `clap` (feature `derive`) | Argumentos: ciudad, coordenadas, unidades |
| 7 | `dotenvy` | Cargar la API key desde `.env` (Visual Crossing o WeatherAPI) |
| 8 | `comfy-table` / `colored` | Mostrar la comparación en la terminal |

### Opción A: con openmeteo-rs

```toml
[dependencies]
openmeteo-rs = "1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
anyhow = "1"
clap = { version = "4", features = ["derive"] }
```

### Opción B: cliente propio y síncrono

```toml
[dependencies]
ureq = { version = "3", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
clap = { version = "4", features = ["derive"] }
jiff = "0.2"
```

> Comprueba las versiones actuales con `cargo add <crate>`.

---

## 3. Boilerplates y plantillas (ranking)

Para un script de este tamaño, lo más práctico es `cargo new` seguido de `cargo add`. Si prefieres una plantilla, este es el orden recomendado:

| # | Plantilla | Uso | Qué trae |
|---|-----------|-----|----------|
| 1 | [rust-github/template](https://github.com/rust-github/template) | `cargo generate rust-github/template` | Plantilla genérica para proyectos en GitHub, con CI |
| 2 | [rust-starter](https://github.com/rust-starter/rust-starter) | clonar o cargo-generate | clap, config-rs, logging, CI/CD, Justfile. Algunas dependencias están anticuadas (failure, slog) |
| 3 | [eugene-yzm/rust-cli-template](https://github.com/eugene-yzm/rust-cli-template) | `cargo generate eugene-yzm/rust-cli-template` | CLI mínima con clap |
| 4 | [IzawGithub/template_rust](https://github.com/IzawGithub/template_rust) | `cargo generate --git https://github.com/IzawGithub/template_rust` | Just y git-cliff para el changelog |
| 5 | [pjmp/rust-cli-boilerplate](https://github.com/pjmp/rust-cli-boilerplate) | `cargo generate --git https://github.com/pjmp/rust-cli-boilerplate.git` | Solo clap |

Requisito: `cargo install cargo-generate`.

> No he verificado las estrellas ni la actividad reciente de estos repositorios. Revisa la fecha del último commit antes de elegir uno.

---

## 4. Stack recomendado

1. `cargo new weather-diff`
2. `cargo add openmeteo-rs anyhow tokio --features tokio/rt-multi-thread,tokio/macros` y `cargo add clap --features derive`
3. Una sola llamada a Open-Meteo con `current` + `hourly` + `past_days=1`
4. Buscar en `hourly` la entrada de hace 24 h y calcular las diferencias (temperatura, humedad, viento, precipitación)
5. Mostrar el resultado y la atribución a Open-Meteo

### Problemas a tener en cuenta

- **Zona horaria**: sin `timezone=auto`, las horas vienen en UTC.
- **Resolución**: `current` se actualiza cada 15 minutos y `hourly` es por hora. Redondea la hora actual antes de comparar.
- **Modelo frente a estación**: Open-Meteo usa datos de modelo y AEMET mide en estaciones reales. Los valores pueden diferir algo.

---

## 5. Diseño: resumen de las próximas 24 horas en lenguaje natural

**Objetivo**: que el script diga cosas como «esta mañana hace más fresco que ayer», «esta tarde no lloverá» o «esta noche hará mucho más frío que anoche».

### Datos: una sola petición

- Con `past_days=1` y `forecast_days=2` se obtienen 72 horas seguidas: ayer, hoy y mañana, hora a hora.
- Variables:
  - `apparent_temperature`: la sensación térmica, que es lo que la gente percibe. Es preferible a `temperature_2m` para hablar de «más fresco».
  - `precipitation_probability`: la probabilidad de lluvia, para las horas futuras.
  - `precipitation`: los mm de lluvia, para saber si ayer llovió.
  - `weather_code`: despejado, nublado, tormenta…
  - `wind_speed_10m` (opcional): para frases como «más viento que ayer».
- Con `timezone=auto` las franjas salen en hora local.

### Franjas horarias

| Franja | Horas | Cómo se nombra |
|--------|-------|----------------|
| Mañana | 6–12 | «esta mañana» / «mañana por la mañana» |
| Tarde | 12–20 | «esta tarde» / «mañana por la tarde» |
| Noche | 20–6 | «esta noche» |

- Se toman solo las franjas que caen en las **próximas 24 horas**, en orden cronológico.
  - Ejemplo a las 18:00: lo que queda de esta tarde, después esta noche y después mañana por la mañana.
- Cada franja se compara con **la misma franja de ayer** (tarde con tarde, noche con noche).
- Si la franja está a medias, como «esta tarde» a las 18:00, se compara solo el tramo horario equivalente de ayer.

### Umbrales de temperatura

Se calcula la media de sensación térmica de la franja de hoy menos la de ayer:

| Diferencia | Frase |
|------------|-------|
| menos de 1 °C | «parecido a ayer» (o no se menciona) |
| de 1 a 3 °C | «algo más fresco» / «algo más templado» |
| de 3 a 6 °C | «más fresco» / «más calor» |
| más de 6 °C | «mucho más frío» / «mucho más calor» |

### Umbrales de lluvia

Se mira la probabilidad máxima de la franja:

| Probabilidad | Frase |
|--------------|-------|
| menos del 20 % | «no lloverá» |
| del 20 al 50 % | «puede caer algo» |
| más del 50 % | «lloverá» |

**Contrastes con ayer**, que merecen frase propia:
- Si ayer llovió y hoy no: «hoy no se repetirá la lluvia de ayer».
- Si ayer estuvo seco y hoy hay lluvia probable: «a diferencia de ayer, lloverá».

### Reglas de redacción

- **Solo lo relevante**: si una franja no cambia respecto a ayer, se omite o se agrupa («el resto del día, similar a ayer»).
- Se combina temperatura y lluvia en una sola frase por franja.
- Se puede añadir la diferencia numérica entre paréntesis como detalle (−4 °C, 40 %).
- Al final se pone la atribución a Open-Meteo.

### Ejemplo de salida

> Esta tarde hace bastante más fresco que ayer (−4 °C) y no lloverá.
> Esta noche será mucho más fría que anoche.
> Mañana por la mañana, parecido a hoy, con posibilidad de lluvia (40 %).
>
> *Datos meteorológicos de Open-Meteo.com*

### Limitaciones

- Los datos de ayer que da Open-Meteo proceden del modelo, no de estaciones. Para este uso son suficientes.
- Los umbrales son orientativos y conviene ajustarlos con el uso. Por ejemplo, en verano 3 °C de diferencia se notan menos que en invierno.

---

## Fuentes

- Open-Meteo, documentación: https://open-meteo.com/en/docs
- Open-Meteo, API histórica: https://open-meteo.com/en/docs/historical-weather-api
- openmeteo-rs: https://docs.rs/openmeteo-rs
- Visual Crossing: https://www.visualcrossing.com/weather-api
- WeatherAPI.com: https://www.weatherapi.com/docs/
- AEMET OpenData: https://opendata.aemet.es/centrodedescargas/inicio
- Comparativa de APIs (ostechnix): https://ostechnix.com/best-free-weather-apis-linux/
- Comparativa de APIs (Geekflare): https://geekflare.com/guides/weather-api/
- Límites de WeatherAPI: https://github.com/api-evangelist/weatherapi
- Límites de Visual Crossing: https://aiagentsdirectory.com/blog/the-best-weather-api-providers-for-ai-agents
