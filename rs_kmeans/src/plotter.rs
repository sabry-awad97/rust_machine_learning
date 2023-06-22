use std::error::Error;

use plotters::prelude::*;

pub struct DataPlotter<'a> {
    file_name: &'a str,
    data: &'a [Vec<f64>],
    x_range: (f64, f64),
    y_range: (f64, f64),
    point_size: i32,
    point_color: RGBColor,
    point_label_font: FontDesc<'static>,
}

#[allow(unused)]
impl<'a> DataPlotter<'a> {
    pub fn new(file_name: &'a str, data: &'a [Vec<f64>]) -> Self {
        Self {
            file_name,
            data,
            x_range: (0.0, 20.0),
            y_range: (0.0, 10.0),
            point_size: 5,
            point_color: RED,
            point_label_font: ("sans-serif", 12).into_font(),
        }
    }

    pub fn x_range(mut self, range: (f64, f64)) -> Self {
        self.x_range = range;
        self
    }

    pub fn y_range(mut self, range: (f64, f64)) -> Self {
        self.y_range = range;
        self
    }

    pub fn point_size(mut self, size: i32) -> Self {
        self.point_size = size;
        self
    }

    pub fn point_color(mut self, color: RGBColor) -> Self {
        self.point_color = color;
        self
    }

    pub fn point_label_font(mut self, font: FontDesc<'static>) -> Self {
        self.point_label_font = font;
        self
    }

    pub fn plot(&self) -> Result<(), Box<dyn Error>> {
        let root_area = BitMapBackend::new(self.file_name, (1024, 768)).into_drawing_area();
        root_area.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root_area)
            .margin(10)
            .caption("K-Means Clustering", ("sans-serif", 20))
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                self.x_range.0..self.x_range.1,
                self.y_range.0..self.y_range.1,
            )?;

        chart.configure_mesh().x_labels(20).y_labels(10).draw()?;

        let data_points: Vec<(f64, f64)> = self.data.iter().map(|row| (row[0], row[1])).collect();

        chart.draw_series(PointSeries::of_element(
            data_points,
            self.point_size,
            ShapeStyle::from(&self.point_color).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(
                        format!("{:?}", coord),
                        (5, 5),
                        self.point_label_font.clone(),
                    )
            },
        ))?;

        root_area.present()?;
        Ok(())
    }
}

pub fn plot_3d_data(file_name: &str, data: &[Vec<f64>]) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new(file_name, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .margin(10)
        .caption("K-Means Clustering", ("sans-serif", 20))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_3d(0.0..11.0, 0.0..5.0, 0.0..10.0)?;

    chart
        .configure_axes()
        .x_labels(11)
        .y_labels(5)
        .z_labels(10)
        .draw()?;

    let data_points: Vec<(f64, f64, f64)> =
        data.iter().map(|row| (row[0], row[1], row[2])).collect();

    chart.draw_series(PointSeries::of_element(
        data_points,
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
                + Text::new(
                    format!("{:?}", coord),
                    (5, 5),
                    ("sans-serif", 12).into_font(),
                )
        },
    ))?;

    root_area.present()?;

    Ok(())
}
