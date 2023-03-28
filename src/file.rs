use calamine::{open_workbook, Xlsx, Reader, DataType, Range};

const PATH:&str = "statistics/fdic.xlsx";

#[derive(Debug, Clone, Copy)]
struct FdicColumns {
    index_of_name: usize,
    index_of_year: usize,
    index_of_year_stop: usize
}

#[derive(Debug)]
struct FdicStat{
    name: String,
    stats: Vec<f64>
}

#[derive(Debug)]
pub struct FdicStats {
    columns: FdicColumns,
    stats: Vec<FdicStat>
}


pub fn parse_xls() -> Result<Option<FdicStats>, Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(PATH)?;
    println!("{:?}", excel.sheet_names());
    if let Some(Ok(r)) = excel.worksheet_range("FDIC") {
        return Ok(Some(FdicStats::new(r)));
    }
    Ok(None)
}


impl FdicStat {

    fn new(name: String, row: &[DataType]) -> FdicStat {
        let mut stats = vec![];
        for col in row {
            if !col.is_empty() {
                if let Ok(r) = col.to_string().parse::<f64>() {
                    stats.push(r);
                }
            }
        }
        FdicStat { name, stats }
    }
}


impl FdicColumns {
    fn new(row: &[DataType]) -> Option<Self> {
        let mut is_row = false;
        let mut parsed_int = false;
        let mut index_of_name = 0;
        let mut index_of_year = 0;
        let mut index_of_year_stop = 0;
        for (i, col) in row.iter().enumerate() {
            match Self::start_stat(col.to_string().as_str())  {
                true => {
                    is_row = true;
                    index_of_name = i;
                },
                _ => {}
            }
            if is_row && !parsed_int {
                if let Ok(_) = col.to_string().parse::<u32>() {
                    index_of_year = i;
                    parsed_int = true;
                }
            } else if is_row && parsed_int {
                if let Ok(year) = col.to_string().parse::<u32>() {
                    if year == 2002 {
                        index_of_year_stop = i;
                    }
                }
            }
        }
        if index_of_year != 0 {
            Some(FdicColumns {
                index_of_name,
                index_of_year,
                index_of_year_stop
            })
        } else {
            None
        }
    }
    
    fn start_stat(s: &str) -> bool {
        s == "Dollar Amounts in Billions"
    }
    fn end_stat(s: &str) -> bool {
        s.contains("(Includes RTC before 1996)")
    }
}

impl FdicStats {
    fn new<'a>(r: Range<DataType>) -> Self {
        let mut fdic_col: Option<FdicColumns> = None;
        let mut fdic_stats: Vec<FdicStat> = vec![];
        let mut reached_column = false;
        for row in r.rows() {
            if !reached_column {
                let col = FdicColumns::new(row);
                if col.is_some() {
                    fdic_col = col;
                    reached_column = true;
                }
            } else if reached_column {
                if let Some(col) = fdic_col {
                    if FdicColumns::end_stat(row[col.index_of_name].to_string().as_str()) {
                        break;
                    } else if !row[col.index_of_name].is_empty() {
                        let new_stat = FdicStat::new(row[col.index_of_name].to_string(), &row[col.index_of_year..col.index_of_year_stop]);
                        fdic_stats.push(new_stat);
                    }
                }
            }
        }
        println!("{:?}", &fdic_stats[fdic_stats.len() - 1]);
        FdicStats{ columns: fdic_col.unwrap(), stats: fdic_stats }
    }
}

