extern crate selenium_rs;

use selenium_rs::element::Element;
use selenium_rs::webdriver::*;

fn get_search_bar(driver: &mut WebDriver) -> Element {
    driver.start_session().unwrap();
    driver.navigate("http://google.com").unwrap();
    driver
        .find_element(Selector::CSS, "input[maxlength=\"2048\"]")
        .unwrap()
}

#[test]
fn test_enter_text() {
    let mut webdriver = WebDriver::new(Browser::Chrome);
    let search_bar = get_search_bar(&mut webdriver);
    assert!(search_bar.type_text("testing").is_ok());
}

#[test]
fn test_search() {
    let mut webdriver = WebDriver::new(Browser::Chrome);
    get_search_bar(&mut webdriver).type_text("testing").unwrap();
    let google_search_button = webdriver
        .find_element(Selector::CSS, "input[name=\"btnK\"]")
        .unwrap();
    assert!(google_search_button.click().is_ok());
    assert!(google_search_button.click().is_ok());
}
