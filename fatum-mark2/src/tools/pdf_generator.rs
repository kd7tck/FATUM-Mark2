use genpdf::{elements, style, fonts, Element};
use anyhow::Result;
use crate::tools::feng_shui::FengShuiReport;

pub fn generate_pdf(report: &FengShuiReport) -> Result<Vec<u8>> {
    let font_family = fonts::from_files("assets/fonts", "Roboto", None)
        .unwrap_or_else(|_| fonts::from_files("./", "Roboto", None)
        .unwrap_or_else(|_| fonts::from_files("/usr/share/fonts/truetype/dejavu", "DejaVuSans", None).unwrap()));

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Fatum Feng Shui Report");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // Title
    doc.push(elements::Paragraph::new("FATUM-MARK2 QUANTUM FENG SHUI REPORT")
        .styled(style::Style::new().bold().with_font_size(20)));
    doc.push(elements::Break::new(1.5));

    // BaZi
    if let Some(bazi) = &report.bazi {
        doc.push(elements::Paragraph::new("BAZI FOUR PILLARS").styled(style::Style::new().bold()));
        let mut table = elements::TableLayout::new(vec![1, 1, 1, 1]);
        table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
        table.row().element(elements::Paragraph::new("Year")).element(elements::Paragraph::new("Month"))
             .element(elements::Paragraph::new("Day")).element(elements::Paragraph::new("Hour")).push().expect("Invalid table");
        table.row().element(elements::Paragraph::new(&bazi.year_pillar))
             .element(elements::Paragraph::new(&bazi.month_pillar))
             .element(elements::Paragraph::new(&bazi.day_pillar))
             .element(elements::Paragraph::new(&bazi.hour_pillar))
             .push().expect("Invalid table");
        doc.push(table);
        doc.push(elements::Break::new(1.0));
    }

    // Flying Stars
    doc.push(elements::Paragraph::new(format!("FLYING STARS: {}", report.annual_chart.label)).styled(style::Style::new().bold()));
    doc.push(elements::Paragraph::new(format!("Facing: {} | Sitting: {}", report.annual_chart.facing_mountain, report.annual_chart.sitting_mountain)));

    // Grid 3x3
    let grid_indices = [
        [8, 4, 6],
        [7, 0, 2],
        [3, 5, 1]
    ];
    let mut grid = elements::TableLayout::new(vec![1, 1, 1]);
    grid.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    for r in 0..3 {
        let mut row = grid.row();
        for c in 0..3 {
            let idx = grid_indices[r][c];
            let p = &report.annual_chart.palaces[idx];
            let text = format!("{}\nB:{} M:{} W:{}", p.sector, p.base_star, p.mountain_star, p.water_star);
            row.push_element(elements::Paragraph::new(text));
        }
        row.push().expect("Table row error");
    }
    doc.push(grid);

    // San He
    if let Some(sh) = &report.san_he {
        doc.push(elements::Break::new(1.0));
        doc.push(elements::Paragraph::new("SAN HE WATER METHOD").styled(style::Style::new().bold()));
        doc.push(elements::Paragraph::new(format!("Method: {}", sh.water_method)));
        doc.push(elements::Paragraph::new("Warnings:"));
        for w in &sh.lucky_water_exit {
            doc.push(elements::Paragraph::new(format!("- {}", w)));
        }
    }

    let mut buffer = Vec::new();
    doc.render(&mut buffer)?;
    Ok(buffer)
}
