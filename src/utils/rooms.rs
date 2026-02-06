#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnseirbRoom {
    TD01,
    TD02,
    TD03,
    TD04,
    TD05,
    TD06,
    TD07,
    TD08,
    TD09,
    TD10,
    TD11,
    TD12,
    TD13,
    TD14,
    TD15,
    TD16,
    TD17,
    TD18,
    TD19,
    TD20,
    TD21,
    TD22,
    TD23,
    TD24,
    TD25,
    TD26,
    TD27,
    TD28,
}

unsafe impl Sync for EnseirbRoom {}

unsafe impl Send for EnseirbRoom {}

impl EnseirbRoom {
    pub fn name(&self) -> Option<String> {
        match self {
            EnseirbRoom::TD01 => None,
            EnseirbRoom::TD02 => None,
            EnseirbRoom::TD03 => None,
            EnseirbRoom::TD04 => Some("EA-S101/S102 (TD04)".to_string()),
            EnseirbRoom::TD05 => Some("EA-S104/S105 (TD05)".to_string()),
            EnseirbRoom::TD06 => Some("EA-S106/S107 (TD06)".to_string()),
            EnseirbRoom::TD07 => Some("EA-S108/S109 (TD07)".to_string()),
            EnseirbRoom::TD08 => Some("EA-S110/S111 (TD08)".to_string()),
            EnseirbRoom::TD09 => Some("EA-S112/S113 (TD09)".to_string()),
            EnseirbRoom::TD10 => Some("EA-S114 (TD10)".to_string()),
            EnseirbRoom::TD11 => Some("EA-S115/S116 (TD11)".to_string()),
            EnseirbRoom::TD12 => Some("EA-S117/S118 (TD12)".to_string()),
            EnseirbRoom::TD13 => Some("EA-S119/S120 (TD13)".to_string()),
            EnseirbRoom::TD14 => Some("EA-S121/S122 (TD14)".to_string()),
            EnseirbRoom::TD15 => Some("EA-S225 (TD15)".to_string()),
            EnseirbRoom::TD16 => None,
            EnseirbRoom::TD17 => Some("EA-S008/S009 (TD17)".to_string()),
            EnseirbRoom::TD18 => None,
            EnseirbRoom::TD19 => None,
            EnseirbRoom::TD20 => Some("EB-P010/P011 (TD20)".to_string()),
            EnseirbRoom::TD21 => Some("EB-P117 (TD21)".to_string()),
            EnseirbRoom::TD22 => Some("EB-P118/P119 (TD22)".to_string()),
            EnseirbRoom::TD23 => Some("EB-P121 (TD23)".to_string()),
            EnseirbRoom::TD24 => Some("EB-P123 (TD24)".to_string()),
            EnseirbRoom::TD25 => Some("EB-P145 (TD25)".to_string()),
            EnseirbRoom::TD26 => Some("EB-P147 (TD26)".to_string()),
            EnseirbRoom::TD27 => Some("EB-P148/P150 (TD27)".to_string()),
            EnseirbRoom::TD28 => Some("EB-P153/P156 (TD28)".to_string()),
        }
    }

    pub fn short_name(&self) -> String {
        match self {
            EnseirbRoom::TD01 => "TD01".to_string(),
            EnseirbRoom::TD02 => "TD02".to_string(),
            EnseirbRoom::TD03 => "TD03".to_string(),
            EnseirbRoom::TD04 => "TD04".to_string(),
            EnseirbRoom::TD05 => "TD05".to_string(),
            EnseirbRoom::TD06 => "TD06".to_string(),
            EnseirbRoom::TD07 => "TD07".to_string(),
            EnseirbRoom::TD08 => "TD08".to_string(),
            EnseirbRoom::TD09 => "TD09".to_string(),
            EnseirbRoom::TD10 => "TD10".to_string(),
            EnseirbRoom::TD11 => "TD11".to_string(),
            EnseirbRoom::TD12 => "TD12".to_string(),
            EnseirbRoom::TD13 => "TD13".to_string(),
            EnseirbRoom::TD14 => "TD14".to_string(),
            EnseirbRoom::TD15 => "TD15".to_string(),
            EnseirbRoom::TD16 => "TD16".to_string(),
            EnseirbRoom::TD17 => "TD17".to_string(),
            EnseirbRoom::TD18 => "TD18".to_string(),
            EnseirbRoom::TD19 => "TD19".to_string(),
            EnseirbRoom::TD20 => "TD20".to_string(),
            EnseirbRoom::TD21 => "TD21".to_string(),
            EnseirbRoom::TD22 => "TD22".to_string(),
            EnseirbRoom::TD23 => "TD23".to_string(),
            EnseirbRoom::TD24 => "TD24".to_string(),
            EnseirbRoom::TD25 => "TD25".to_string(),
            EnseirbRoom::TD26 => "TD26".to_string(),
            EnseirbRoom::TD27 => "TD27".to_string(),
            EnseirbRoom::TD28 => "TD28".to_string(),
        }
    }

    pub fn id(&self) -> Option<u16> {
        match self {
            EnseirbRoom::TD01 => Some(3224),
            EnseirbRoom::TD02 => Some(3223),
            EnseirbRoom::TD03 => Some(3222),
            EnseirbRoom::TD04 => Some(3260),
            EnseirbRoom::TD05 => Some(3259),
            EnseirbRoom::TD06 => Some(3258),
            EnseirbRoom::TD07 => Some(3254),
            EnseirbRoom::TD08 => Some(3253),
            EnseirbRoom::TD09 => Some(3252),
            EnseirbRoom::TD10 => Some(3251),
            EnseirbRoom::TD11 => Some(3250),
            EnseirbRoom::TD12 => Some(3249),
            EnseirbRoom::TD13 => Some(3248),
            EnseirbRoom::TD14 => Some(3247),
            EnseirbRoom::TD15 => Some(3280),
            EnseirbRoom::TD16 => None,
            EnseirbRoom::TD17 => Some(3230),
            EnseirbRoom::TD18 => None,
            EnseirbRoom::TD19 => None,
            EnseirbRoom::TD20 => Some(3296),
            EnseirbRoom::TD21 => Some(3329),
            EnseirbRoom::TD22 => Some(3330),
            EnseirbRoom::TD23 => Some(3331),
            EnseirbRoom::TD24 => Some(3327),
            EnseirbRoom::TD25 => Some(3314),
            EnseirbRoom::TD26 => Some(3315),
            EnseirbRoom::TD27 => Some(3316),
            EnseirbRoom::TD28 => Some(3318),
        }
    }

    pub fn from_string(val: String) -> Option<Self> {
        match val.to_lowercase().as_str() {
            "1" | "td1" => Some(Self::TD01),
            "2" | "td2" => Some(Self::TD02),
            "3" | "td3" => Some(Self::TD03),
            "4" | "td4" => Some(Self::TD04),
            "5" | "td5" => Some(Self::TD05),
            "6" | "td6" => Some(Self::TD06),
            "7" | "td7" => Some(Self::TD07),
            "8" | "td8" => Some(Self::TD08),
            "9" | "td9" => Some(Self::TD09),
            "10" | "td10" => Some(Self::TD10),
            "11" | "td11" => Some(Self::TD11),
            "12" | "td12" => Some(Self::TD12),
            "13" | "td13" => Some(Self::TD13),
            "14" | "td14" => Some(Self::TD14),
            "15" | "td15" => Some(Self::TD15),
            "16" | "td16" => Some(Self::TD16),
            "17" | "td17" => Some(Self::TD17),
            "18" | "td18" => Some(Self::TD18),
            "19" | "td19" => Some(Self::TD19),
            "20" | "td20" => Some(Self::TD20),
            "21" | "td21" => Some(Self::TD21),
            "22" | "td22" => Some(Self::TD22),
            "23" | "td23" => Some(Self::TD23),
            "24" | "td24" => Some(Self::TD24),
            "25" | "td25" => Some(Self::TD25),
            "26" | "td26" => Some(Self::TD26),
            "27" | "td27" => Some(Self::TD27),
            "28" | "td28" => Some(Self::TD28),
            &_ => None,
        }
    }

    pub fn url(&self, start_date: String, end_date: String) -> Option<String> {
        let resid = self.id();
        match resid {
            Some(id) => {
                return Some(format!("https://adeapp.bordeaux-inp.fr/jsp/custom/modules/plannings/anonymous_cal.jsp?resources={id}&projectId=1&calType=ical&firstDate={start_date}&lastDate={end_date}&displayConfigId=71"));
            }
            None => None,
        }
    }
}
