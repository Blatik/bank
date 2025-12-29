use crate::models::{Indicator, IndicatorCategory};

pub fn get_indicators() -> IndicatorCategory {
    IndicatorCategory {
        economic: vec![
            Indicator {
                id: "NY.GDP.MKTP.CD".to_string(),
                name: "GDP (current US$)".to_string(),
                category: "economic".to_string(),
                unit: "USD".to_string(),
            },
            Indicator {
                id: "NY.GDP.PCAP.CD".to_string(),
                name: "GDP per capita (current US$)".to_string(),
                category: "economic".to_string(),
                unit: "USD".to_string(),
            },
            Indicator {
                id: "FP.CPI.TOTL.ZG".to_string(),
                name: "Inflation, consumer prices (annual %)".to_string(),
                category: "economic".to_string(),
                unit: "%".to_string(),
            },
            Indicator {
                id: "GC.DOD.TOTL.GD.ZS".to_string(),
                name: "Government debt (% of GDP)".to_string(),
                category: "economic".to_string(),
                unit: "%".to_string(),
            },
        ],
        demographic: vec![
            Indicator {
                id: "SP.POP.TOTL".to_string(),
                name: "Total population".to_string(),
                category: "demographic".to_string(),
                unit: "people".to_string(),
            },
            Indicator {
                id: "SP.URB.TOTL.IN.ZS".to_string(),
                name: "Urban population (% of total)".to_string(),
                category: "demographic".to_string(),
                unit: "%".to_string(),
            },
            Indicator {
                id: "SP.DYN.CDRT.IN".to_string(),
                name: "Death rate (per 1,000 people)".to_string(),
                category: "demographic".to_string(),
                unit: "per 1,000".to_string(),
            },
            Indicator {
                id: "SP.DYN.CBRT.IN".to_string(),
                name: "Birth rate (per 1,000 people)".to_string(),
                category: "demographic".to_string(),
                unit: "per 1,000".to_string(),
            },
        ],
        social: vec![
            Indicator {
                id: "SE.ADT.LITR.ZS".to_string(),
                name: "Literacy rate (% of population)".to_string(),
                category: "social".to_string(),
                unit: "%".to_string(),
            },
            Indicator {
                id: "SE.ADT.LITR.FE.ZS".to_string(),
                name: "Literacy rate, female (% of females)".to_string(),
                category: "social".to_string(),
                unit: "%".to_string(),
            },
            Indicator {
                id: "SE.ADT.LITR.MA.ZS".to_string(),
                name: "Literacy rate, male (% of males)".to_string(),
                category: "social".to_string(),
                unit: "%".to_string(),
            },
            Indicator {
                id: "SL.UEM.TOTL.ZS".to_string(),
                name: "Unemployment, total (% of labor force)".to_string(),
                category: "social".to_string(),
                unit: "%".to_string(),
            },
        ],
        environmental: vec![
            Indicator {
                id: "EN.ATM.CO2E.PC".to_string(),
                name: "CO2 emissions (metric tons per capita)".to_string(),
                category: "environmental".to_string(),
                unit: "metric tons".to_string(),
            },
            Indicator {
                id: "EG.USE.PCAP.KG.OE".to_string(),
                name: "Energy use (kg of oil equivalent per capita)".to_string(),
                category: "environmental".to_string(),
                unit: "kg".to_string(),
            },
            Indicator {
                id: "AG.LND.FRST.ZS".to_string(),
                name: "Forest area (% of land area)".to_string(),
                category: "environmental".to_string(),
                unit: "%".to_string(),
            },
            Indicator {
                id: "NY.ADJ.RNRW.GN.ZS".to_string(),
                name: "Adjusted savings: natural resources depletion (% GNI)".to_string(),
                category: "environmental".to_string(),
                unit: "%".to_string(),
            },
        ],
    }
}
