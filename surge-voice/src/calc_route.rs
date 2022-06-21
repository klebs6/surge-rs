crate::ix!();

impl SurgeVoice {

    pub fn calc_routes(&mut self, cfg: VoiceRuntimeHandle) {

        let cfg = cfg.borrow();

        macro_rules! routefilter {
            ($route:expr, $cfg:expr) => {{
                let mut route = $route;
                match $cfg {
                    FilterBlockConfiguration::Serial1 | 
                        FilterBlockConfiguration::Serial2 | 
                        FilterBlockConfiguration::Serial3  => {
                            if route == 1 {
                                route = 0;
                            }
                        },
                    _ => {},
                };
                route
            }}
        }

        self.route[0] = routefilter!(cfg.oscillator_route0, cfg.filterblock_cfg);
        self.route[1] = routefilter!(cfg.oscillator_route1, cfg.filterblock_cfg);
        self.route[2] = routefilter!(cfg.oscillator_route2, cfg.filterblock_cfg);
        self.route[3] = routefilter!(cfg.ring_route0,       cfg.filterblock_cfg);
        self.route[4] = routefilter!(cfg.ring_route1,       cfg.filterblock_cfg);
        self.route[5] = routefilter!(cfg.noise_route0,      cfg.filterblock_cfg);
    }
}
