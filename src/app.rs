use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use leptos_struct_table::*;
use async_trait::async_trait;
use std::ops::Range;
use std::time::Duration;
use chrono::{Datelike, NaiveTime, NaiveDateTime, NaiveDate, offset};
use num_traits::cast::FromPrimitive;
use std::collections::VecDeque;
use crate::tailwind::TailwindClassesPreset;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);


    view! { cx,
        <Stylesheet href="/styles.css"/>
        <Router>
            <body class="text-light bg-dark-700">
                <NavBar>
                    <div>"navbar text here"</div>
                </NavBar>
                <main class="my-20 md:mx-8">
                    <Routes>
                        //<Route path="/bets/:limit" view=Bets/>
                        <Route path="/bets" view=move |cx| view! { cx, <Redirect path=format!("/bets/{}", offset::Utc::now().date_naive())/> }/>
                        <Route path="/bets/:date" view=Bets/>
                        <Route path="/favicon.ico" view=move |_| view! { cx, "not found" }/>
                    </Routes>
                </main>
            </body>
        </Router>
    }
}

#[component]
fn NavBar(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let (scrollY, setScrollY) = create_signal(cx, 0f64);
    let (prevScrollY, setPrevScrollY) = create_signal(cx, 0f64);
    let last_result = create_rw_signal(cx, true);
    let showNav = move || {
        if (scrollY() - prevScrollY.get_untracked()).abs() < 40f64 {
            return last_result.get_untracked();
        }
        if scrollY() > prevScrollY.get_untracked() {
            setPrevScrollY(scrollY());
            last_result.set(false);
            return false
        }
        setPrevScrollY(scrollY());
        last_result.set(true);
        return true
    };
    #[cfg(not(feature = "ssr"))]
    set_timeout(move || window_event_listener(ev::scroll, move |_| {
        setScrollY(window().scroll_y().unwrap_or_default());
    }), Duration::from_millis(20));
    view! { cx,
        <nav class="fixed h-12 w-full text-light bg-black transition-all z-50 -top-12" class=("top-0", showNav)>
            {children(cx)}
        </nav>
    }
}

#[component]
fn DateDropdown(cx: Scope, date_selection: RwSignal<NaiveDate>) -> impl IntoView {
    let today = offset::Utc::now().date_naive();
    let params = use_params_map(cx);
    match params.with(|p| p.get("date").cloned()) {
        Some(val) => date_selection.set(val.parse::<NaiveDate>().unwrap_or(today)),
        None => ()
    }
    #[cfg(not(feature = "ssr"))] {
        window().history().unwrap().replace_state(&date_selection.get_untracked().to_string().into(), ""); 
    }
    window_event_listener(ev::popstate, move |ev| {
        ev.prevent_default();
        date_selection.set(ev.state().as_string().unwrap().parse::<NaiveDate>().unwrap());
        window().history().unwrap().replace_state(&date_selection.get_untracked().to_string().into(), ""); 
        log!("{}", date_selection.get_untracked().to_string());
    });
    view! { cx,
        <details>
            <summary class="w-max">{move || date_selection.get().to_string()}</summary>
            <ul>
                {move || today.iter_days().rev().take(10)
                    .filter(|n| *n != date_selection.get())
                    .map(|n| view! { cx, <li class="cursor-pointer"> <button on:click=move |ev| {ev.prevent_default(); date_selection.set(n); window().history().unwrap().push_state_with_url(&date_selection.get_untracked().to_string().into(), "", Some(&format!("/bets/{}", date_selection.get_untracked())));}>{n.to_string()}</button></li>})
                    .collect_view(cx)
                }
            </ul>
        </details>
    }
}

#[component]
fn CalendarDateSelector(cx: Scope, date_selection: RwSignal<NaiveDate>) -> impl IntoView {
    let today = offset::Utc::now().date_naive();
    let params = use_params_map(cx);
    match params.with(|p| p.get("date").cloned()) {
        Some(val) => date_selection.set(val.parse::<NaiveDate>().unwrap_or(today)),
        None => ()
    }
    #[cfg(not(feature = "ssr"))] {
        window().history().unwrap().replace_state(&date_selection.get_untracked().to_string().into(), ""); 
    }
    let view_date: RwSignal<NaiveDate> = create_rw_signal(cx, date_selection.get_untracked());
    window_event_listener(ev::popstate, move |ev| {
        ev.prevent_default();
        date_selection.set(ev.state().as_string().unwrap().parse::<NaiveDate>().unwrap());
        view_date.set(date_selection.get_untracked());
        window().history().unwrap().replace_state(&date_selection.get_untracked().to_string().into(), ""); 
        log!("{}", date_selection.get_untracked().to_string());
    });
    view! { cx,
        <div class="flex flex-col h-[216px]">
            <div class="flex-auto">
                <h3 class="absolute top-0">{move || view_date.with(|d| date_to_month_str(d))}</h3>
            </div>
            //<div class="flex-auto"/>
            <div class="flex-1 grid grid-cols-7 gap-1 h-max"> {
                move || {
                    let mut views: Vec<View> = Vec::with_capacity(31);
                    let current_month = view_date.with(|d| d.month());
                    let current_year = view_date.with(|d| d.year());
                    let current_date = NaiveDate::from_ymd_opt(current_year, current_month, 1u32).unwrap();
                    let iter_days = current_date.iter_days();
                    let pad_amount = current_date.weekday().num_days_from_sunday();
                    iter_days
                        .take(31)
                        .filter(|d| d.month() == current_month)
                        .for_each(|d| views.push(view! { cx,
                                <button 
                                    style=("grid-column-start", move || if d.day() == 1 { (pad_amount + 1).to_string() } else { "auto".to_string() })
                                    class=("bg-dark-300", move || { d == date_selection.get() })
                                    on:click=move |_| {
                                        date_selection.set(d);
                                        window().history().unwrap().push_state_with_url(&date_selection.get_untracked().to_string().into(), "", Some(&format!("/bets/{}", date_selection.get_untracked())));
                                    }
                                >
                                    {d.day()}
                                </button>
                                //b.class("button", format!("col-start-{}", pad_amount + 1))
                            }.into_view(cx)));
                    //views[0] = views[0].clone().into_html_element(cx).unwrap().style("grid-column-start", move || pad_amount + 1).into_view(cx);
                    return views;
                }
            } </div>
            <div class="flex gap-8 justify-center">
                <button on:click=move |_| view_date.set(view_date.get_untracked() - chrono::Months::new(1u32))>"<<"</button>
                <button on:click=move |_| view_date.set(view_date.get_untracked() + chrono::Months::new(1u32))>">>"</button>
            </div>
        </div>
    }
}

fn date_to_month_str(date: &NaiveDate) -> String {
    use chrono::Month;
    return Month::from_u32(date.month()).unwrap().name().to_owned()
}

#[component]
fn Bets(cx: Scope) -> impl IntoView {
    let date = create_rw_signal(cx, offset::Utc::now().date_naive());
    //let limit = create_rw_signal(cx, path_limit);

    view! { cx,
        <div class="relative md:flex z-0">
            <div class="flex-none w-48 absolute md:static z-10 top-0">
                <CalendarDateSelector date_selection=date/>
            </div>
            //<BetsTable date=date.read_only()/>
            <div class="overflow-auto absolute md:static ml-6 top-6" style:height="66vh">
                <BetsTable date=date.read_only()/>
            </div>
        </div>
        //<div>
            //{if path_limit > 0 {
                //Some(view! { cx, 
                        //<a href=format!("/bets/{}", path_limit - 1) target="_self">
                            //"-1"
                        //</a>
                    //})
                //} else {
                    //None
                //}
            //}
            //<a href=format!("/bets/{}", path_limit + 1) target="_self">
                //"+1"
            //</a>
        //</div>
    }
}

#[component]
fn BetsTable(cx: Scope, date: ReadSignal<NaiveDate>) -> impl IntoView {
    //let date = match params.with(|p| p.get("date").cloned()) {
        //Some(val) => {
           //create_rw_signal(cx,  val.parse().unwrap_or(offset::Utc::now().date_naive()))
        //},
        //None => {
            //create_rw_signal(cx, offset::Utc::now().date_naive())
        //}
    //};
    //let params = use_params_map(cx);
    //let path_limit = match use_params_map(cx).with(|p| p.get("limit").cloned()) {
        //Some(val) => val.parse::<u32>().unwrap_or(0u32),
        //None => 0u32
    //};
    //let path_limit = match path_limit() {
        //Some(val) => create_rw_signal(cx, val.parse::<u32>().unwrap_or(0u32)),
        //None => create_rw_signal(cx, 0u32)
    //};
    //window().history().unwrap().set_scroll_restoration(ScrollRestoration::Auto);
    //window_event_listener(ev::popstate, move |ev| {
        //ev.prevent_default();
        //window().location().set_href(&format!("/{}", ev.state().as_f64().unwrap()));
        //window().scroll_to_with_x_and_y(0f64, ev.state().as_f64().unwrap());
        //window().history().unwrap().replace_state(&limit.get_untracked().into(), "");
        //log!("{}", ev.state().as_f64().unwrap());
        //limit.set(ev.state().as_f64().unwrap());
        //limit.set(window().history().unwrap().state().unwrap().as_f64().unwrap());
        //log!("{}", path_limit);
    //});
    let bets = create_rw_signal(cx, BetDataProvider{cx: cx, date: date, sorting: VecDeque::from([(BetColumnName::Rowid, ColumnSort::Descending)]), last_date: Arc::new(Mutex::new(RefCell::new(NaiveDate::MIN))), bets: Arc::new(Mutex::new(RefCell::new(vec![])))});
    let table = move || {
        date.track();
        view! { cx,
            <BetTable items=bets/>
        }
    };
    view! { cx, 
//        <button on:click=move |_| limit.update(|limit| *limit += 1)>"+1"</button>
 //       <button on:click=move |_| limit.update(|limit| *limit -= 1)>"-1"</button>
        //<BetTable items=bets/>
        //<a href=format!("/{}", limit() + 1) target="_self">
        //<button on:click=move |ev| { ev.prevent_default(); window().history().expect("to have history").push_state_with_url(&window().scroll_y().unwrap().into(), "", Some(&format!("/{}", limit.get_untracked() + 1))); limit.update(|limit| *limit += 1); }>
        //TODO fix wrap table head cells in buttons for mobile
        {table}
        //<button on:click=move |ev| { 
            //ev.prevent_default();
            //log!("{}", JsValue::from_f64(limit.get_untracked()).as_f64().unwrap());
            //limit.update(|l| *l += 1f64);
            //window().history().unwrap().push_state_with_url(&limit.get_untracked().into(), "", Some(&format!("/{}", limit.get_untracked()))); 
            //window().history().unwrap().push_state(&limit.get_untracked().into(), ""); 
            //log!("{}", window().history().unwrap().state().unwrap().as_f64().unwrap());
        //}>
            //"+1"
        //</button>
        //</a>
    }
}

#[derive(Clone, TableComponent, Deserialize, Serialize, Debug, PartialEq)]
#[table(sortable, classes_provider = "TailwindClassesPreset")]
pub struct Bet {
    #[table(key)]
    rowid: i64,
    #[table(title = "Time")]
    #[table(format(string = "%H:%M"))]
    dt: NaiveDateTime,
    blue_team: String,
    red_team: String,
    #[table(format(precision = 3))]
    blue_odds: f64,
    #[table(format(precision = 3))]
    red_odds: f64,
}

#[derive(Clone, Debug)]
pub struct BetDataProvider {
    cx: Scope,
    date: ReadSignal<NaiveDate>,
    sorting: VecDeque<(BetColumnName, ColumnSort)>,
    last_date: Arc<Mutex<RefCell<NaiveDate>>>,
    bets: Arc<Mutex<RefCell<Vec<Bet>>>>,
}

impl PartialEq for BetDataProvider {
    fn eq(&self, other: &Self) -> bool {
        return self.date.get_untracked() == other.date.get_untracked()
    }
}

#[async_trait(?Send)]
impl TableDataProvider<Bet> for BetDataProvider {
    type ColumnName = BetColumnName;

    async fn get_rows(&self, range: Range<usize>) -> Vec<Bet> {
        if self.date.get_untracked() != *self.last_date.lock().unwrap().borrow() {
            let res = match get_bets(self.cx, self.date.get_untracked().to_string(), self.sorting.clone()).await {
                Ok(bets) => bets,
                Err(_) => {
                    vec![]
                }
            };
            log!("{:?}\t{:?}", self.date.get_untracked(), *self.last_date.lock().unwrap().borrow());
            *self.last_date.lock().unwrap().borrow_mut() = self.date.get_untracked();
            *self.bets.lock().unwrap().borrow_mut() = get_vec_range_clamped(&res, range);
        } else {
            log!("cached");
        }
        for (name, sort_type) in self.sorting.iter().rev() {
            match *sort_type {
                ColumnSort::Ascending => self.bets.lock().unwrap().borrow_mut().sort_by(|b1, b2| b1.get(*name).partial_cmp(&b2.get(*name)).unwrap()),
                ColumnSort::Descending => self.bets.lock().unwrap().borrow_mut().sort_by(|b1, b2| b2.get(*name).partial_cmp(&b1.get(*name)).unwrap()),
                ColumnSort::None => ()
            };
        }
        return self.bets.lock().unwrap().borrow().clone();
    }
    fn set_sorting(&mut self, sorting: &VecDeque<(Self::ColumnName, ColumnSort)>) {
        if sorting.is_empty() {
            self.sorting = VecDeque::from([(BetColumnName::Rowid, ColumnSort::Descending)])
        } else {
            self.sorting = sorting.clone();
        }
    }
}

#[server(GetBets, "/api", "GetJson", "/get_bets")]
pub async fn get_bets(cx: Scope, date: String, sorting: VecDeque<(BetColumnName, ColumnSort)>) -> Result<Vec<Bet>, ServerFnError> {
    use leptos_actix::*;
    use actix_web::web::*;
    use sqlx::{Pool, Sqlite};
    use actix_web::http::*;
    use actix_web::http::header::{CacheControl, Header, TryIntoHeaderValue};
    use actix_web::http::header::*;

    let response = expect_context::<ResponseOptions>(cx);
    //if limit < 3 {
        //response.set_status(StatusCode::NOT_MODIFIED);
    //}
    //response.insert_header(CacheControl::name(), HeaderValue::from_str("public")?);
    let pool = extract(cx, |data: Data<Pool<Sqlite>>| async move { data }).await?;
    //let conn = pool.acquire().await;
    let mut rows: Vec<Bet> = sqlx::query_as!(Bet, r#"SELECT rowid, dt, blue_team, red_team, blue_odds, red_odds FROM bets WHERE date(dt)=? ORDER BY rowid DESC"#, date).fetch_all(pool.as_ref()).await?;
    return Ok(rows)
}

