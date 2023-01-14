macro_rules! fox {
   ($cub:ty) => {{
       let fox: ArcticFox<$cub> =
           Live($cub::default());
       return fox;
   }};
   ($cub:ty, $req:expr) => {{
        let fox: ArcticFox<$cub>;
        if let Ok(ts) = $req::to_model(&$cub) {
            return Live(ts) 
        } else {
            Err(format!("Could not convert request => {} into a model.", $req ))
        }
   }}
}

