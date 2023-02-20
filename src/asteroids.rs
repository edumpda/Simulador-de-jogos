pub mod asteroids{
    use macroquad::prelude::*;
    use std::collections::HashSet;
    use macroquad::rand::gen_range;

    const SHIP_HEIGHT: f32 = 50.;
    const SHIP_BASE: f32 = 62.5;
    const SHIP_LINE_THICKNESS: f32 = 2.;
    const SHIP_ROTATION_SPEED: f32 = 5.;
    const SHIP_MOVING_SPEED: f32 = 4.;
    const SHIELD_SIZE: f32 = 28.;
    const SHIELD_LINE_THICKNESS: f32 = 5.;

    const ASTEROID_COUNT: u8 = 3;
    const ASTEROID_LINE_THICKNESS: f32 = 2.;

    const BULLET_SIZE: f32 = 5.;
    const BULLET_RANGE: f64 = 1.0;
    const BULLET_DELAY: f64 = 0.1;

    const BACKGROUND_COLOR: Color = WHITE;
    const SHIP_COLOR: Color = RED;
    const SHIELD_COLOR: Color = GOLD;
    const ASTEROID_COLOR: Color = LIGHTGRAY;
    const BULLET_COLOR: Color = RED;

    pub const TEXT_COLOR: Color = BLACK;
    pub const TEXT_SIZE: u16 = 40;
    
    struct Ship {
        ship_logo: Texture2D,
        bullet_logo: Texture2D,
        position: Vec2,
        rotation: f32,
        bullets: Vec<Bullet>,
        has_shield: bool,
        last_time_shot: f64,
    }
    
    impl Ship {
        fn new(_ship_logo: Texture2D, _bullet_logo: Texture2D) -> Ship {
            rand::srand(macroquad::miniquad::date::now() as _);
            Ship {
                ship_logo: _ship_logo,
                bullet_logo: _bullet_logo,
                position: Vec2::new(screen_width() / 2., screen_height() / 2.),
                rotation: 0.,
                bullets: vec![],
                has_shield: false,
                last_time_shot: get_time(),
            }
        }
    
        fn draw(&mut self) {
            let rotation = self.rotation.to_radians();
    
            draw_texture_ex(
                self.ship_logo,
                self.position.x - 25.,
                self.position.y - 31.25,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(50., 62.5)),
                    rotation: self.rotation.to_radians(),
                    ..Default::default()
                },
            );
            draw_circle(self.position.x,self.position.y,1.,SHIP_COLOR);
            let rotation_ship = self.rotation;

            let bullet_logo = self.bullet_logo;
    
            self.bullets.iter_mut().for_each(|b| b.update());
            self.bullets.iter_mut().for_each(|b| b.draw(bullet_logo,rotation_ship));
            // range of bullets
            self.bullets
                .retain(|b| get_time() - b.time_shot_out < BULLET_RANGE && !b.collided);
        }
    
        fn mv(&mut self) {
            if !is_key_down(KeyCode::Z) {
                self.has_shield = false;
            }
    
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                let rotation = self.rotation.to_radians();
    
                self.position.y += rotation.cos() * -SHIP_MOVING_SPEED;
                self.position.x += rotation.sin() * SHIP_MOVING_SPEED;
            }
    
            if is_key_pressed(KeyCode::X) {
                self.shoot();
            }
    
            if is_key_down(KeyCode::Z) && !is_key_pressed(KeyCode::X) {
                self.has_shield = true;
                draw_poly_lines(
                    self.position.x,
                    self.position.y,
                    255, // 255 is here to make it a circle since 255 is the max number that this can be
                    SHIELD_SIZE,
                    self.rotation,
                    SHIELD_LINE_THICKNESS,
                    SHIELD_COLOR,
                )
            }
    
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                self.rotation -= SHIP_ROTATION_SPEED;
            }
    
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                self.rotation += SHIP_ROTATION_SPEED;
            }
    
            // wraping:
            if self.position.x > screen_width() {
                self.position.x = 0.;
            } else if self.position.x < 0. {
                self.position.x = screen_width();
            }
    
            if self.position.y > screen_height() {
                self.position.y = 0.;
            } else if self.position.y < 0. {
                self.position.y = screen_height();
            }
        }
    
        fn shoot(&mut self) {
            if get_time() - self.last_time_shot > BULLET_DELAY {
                self.bullets.push(Bullet::new(self.position, self.rotation));
                self.last_time_shot = get_time();
            }
        }
    }
    
    #[derive(Copy, Clone)]
    struct Bullet {
        position: Vec2,
        rotation: f32,
        time_shot_out: f64,
        collided: bool,
    }
    
    impl Bullet {
        fn new(position: Vec2, rotation: f32) -> Bullet {
            Bullet {
                position,
                rotation,
                time_shot_out: get_time(),
                collided: false,
            }
        }
    
        fn draw(&mut self, logo: Texture2D, rot: f32) {

            draw_texture_ex(
                logo,
                self.position.x ,
                self.position.y - SHIP_HEIGHT/2.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(10., 10.)),
                    rotation: rot.to_radians(),
                    ..Default::default()
                },
            );

        }
    
        fn update(&mut self) {
            // spacing of bullets
            let rotation = self.rotation.to_radians();
            self.position.y += rotation.cos() * rand::gen_range(-10., -5.);
            self.position.x += rotation.sin() * rand::gen_range(5., 10.);
        }
    }
    
    struct Asteroid {
        position: Vec2,
        sides: u8,
        size: f32,
        rotation: f32,
    }
    
    impl Asteroid {
        fn new() -> Asteroid {
            Asteroid {
                position: Vec2::new(
                   rand::gen_range(35.0, screen_width() - 36.),
                   rand::gen_range(35.0, screen_height() - 36.),
                //    0.,
                //    0.
                ),
                sides: rand::gen_range(12, 24),
                size: 100.,
                rotation: rand::gen_range(-360.0, 359.),
            }
        }
    
        fn draw(&self, logo: Texture2D) {
    
            draw_texture_ex(
                logo,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(self.size, self.size)),
                    rotation: self.rotation.to_radians(),
                    ..Default::default()
                },
            );
    
        }
    
        fn collided(&self, position: &Vec2) -> bool {
            if (self.position - *position).length() < self.size{
                return true;
            }
            false
        }
    
        fn resize(&mut self) -> Option<Asteroid> {
            let sides_range = match self.sides {
                (12..=24) => {
                    self.sides = rand::gen_range(9, 11);
                    (9, 11)
                }
                (9..=11) => {
                    self.sides = rand::gen_range(6, 8);
                    (6, 8)
                }
                (6..=8) => {
                    self.sides = rand::gen_range(3, 5);
                    (3, 5)
                }
                _ => return None,
            };
    
            let some = Some(Asteroid {
                position: self.position,
                sides: rand::gen_range(sides_range.0, sides_range.1),
                size: self.size / 2.,
                rotation: self.rotation * 3.,
            });
    
            self.size /= 2.;
            self.rotation *= 2.;
   
            some
        }
    
        fn mv(&mut self) {
            let rotation = self.rotation.to_radians();
    
            match self.sides {
                (12..=24) => {
                    let speed = rand::gen_range(4.0, 5.);
                    self.position.y += rotation.cos() * -speed;
                    self.position.x += rotation.sin() * speed;
                }
                (9..=11) => {
                    let speed = rand::gen_range(5.0, 6.);
                    self.position.y += rotation.cos() * -speed;
                    self.position.x += rotation.sin() * speed;
                }
                (6..=8) => {
                    let speed = rand::gen_range(6.0, 7.);
                    self.position.y += rotation.cos() * -speed;
                    self.position.x += rotation.sin() * speed;
                }
                _ => {
                    let speed = rand::gen_range(7.0, 8.);
                    self.position.y += rotation.cos() * -speed;
                    self.position.x += rotation.sin() * speed;
                }
            }
    
            // wraping:
            if self.position.x > screen_width() {
                self.position.x = 0.;
            } else if self.position.x < 0. {
                self.position.x = screen_width();
            }
    
            if self.position.y > screen_height() {
                self.position.y = 0.;
            } else if self.position.y < 0. {
                self.position.y = screen_height();
            }
        }
    }
    
    fn draw_screen(bg_texture: Texture2D, atletico_logo: Texture2D, ship: &mut Ship, asteroids: &Vec<Asteroid>) {
        // Desenha o fundo
        draw_texture_ex(
            bg_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        ship.draw();
        asteroids.iter().for_each(|a| a.draw(atletico_logo));
    }

    pub enum AsteroidsState {
        Startup,
        Lose,
        Win,
        Paused,
        Running
    }

    pub(crate) async fn asteroids_game() -> bool{

        // Textures
        let bg_texture: Texture2D = load_texture("res/asteroids_bg.png").await.unwrap();
        let win_bg_texture: Texture2D = load_texture("res/asteroids_lose_bg.png").await.unwrap();
        let lose_bg_texture: Texture2D = load_texture("res/asteroids_win_bg.png").await.unwrap();
        let bullet_texture: Texture2D = load_texture("res/bola.png").await.unwrap();
        let cruzeiro_logo: Texture2D = load_texture("res/cruzeiro-logo.png").await.unwrap();
        let atletico_logo: Texture2D = load_texture("res/atletico-logo.png").await.unwrap();
        
        // Criação do Ship
        let mut ship = Ship::new(cruzeiro_logo, bullet_texture);

        // Criação dos asteroids
        let mut asteroids: Vec<_> = (0..ASTEROID_COUNT).map(|_| Asteroid::new()).collect();

        // Verificador de colisão
        let mut collided = false;

        // Estado inicial do Asteroids
        let mut game_state = AsteroidsState::Startup;

        loop {
            // Desenha todos os elementos da tela
            draw_screen(bg_texture, atletico_logo, &mut ship, &asteroids);

            match game_state {

                AsteroidsState::Startup => {
                    let text = "Asteroids- Cruzeiro Edition";
                    let font_size = 40.;
                    let text_size = measure_text(text, None, font_size as _, 1.0);
                    draw_text(
                        text,
                        screen_width() / 4. - text_size.width / 2. + 22.,
                        screen_height() / 5. - text_size.height / 2.,
                        font_size,
                        WHITE,
                    );

                    let text = "Pressione (enter) para iniciar";
                    let font_size = 30.;
                    let text_size = measure_text(text, None, font_size as _, 1.0);
                    draw_text(
                        text,
                        screen_width() / 4. - text_size.width / 2. + 22.,
                        screen_height() / 4. - text_size.height / 2.,
                        font_size,
                        WHITE,
                    );

                    let text = "Pressione (Esc) para voltar";
                    let font_size = 30.;
                    let text_size = measure_text(text, None, font_size as _, 1.0);
                    draw_text(
                        text,
                        screen_width() / 4. - text_size.width / 2. + 22.,
                        screen_height() / 4. + 25. - text_size.height / 2.,
                        font_size,
                        WHITE,
                    );

                    if is_key_pressed(KeyCode::Escape) {
                        return false;
                    }

                    if is_key_pressed(KeyCode::Enter) {
                        let mut timer_count = 3;

                        loop {
                            if timer_count == 0 {
                                break;
                            }

                            draw_screen(bg_texture, atletico_logo, &mut ship, &asteroids);

                            let text = &format!("Iniciando em {} ...", timer_count);
                            let text_size = measure_text(text, None, font_size as _, 1.0);
                            draw_text(
                                text,
                                screen_width() / 2. - text_size.width / 2.,
                                screen_height() * 3. / 4. - text_size.height / 2.,
                                font_size,
                                WHITE,
                            );

                            next_frame().await;
                            
                            let old = macroquad::time::get_time();

                            loop {
                                let now = macroquad::time::get_time();
                                if now - old >= 1.0 {
                                    break;
                                }
                            }

                            timer_count -= 1;
                            
                        }

                        game_state = AsteroidsState::Running;
                        continue;
                    }

                    next_frame().await;
                    continue;
                }
    
                AsteroidsState::Paused => {
                    let text = "PAUSADO";
                    let font_size = 60.;
                    let text_size = measure_text(text, None, font_size as _, 1.0);
                    draw_text(
                        text,
                        screen_width() / 4. - text_size.width / 2.,
                        screen_height() / 5. - text_size.height / 2.,
                        font_size,
                        WHITE,
                    );

                    let text2 = "Aperte [esc] para continuar";
                    let font_size = 30.;
                    let text_size = measure_text(text, None, font_size as _, 1.0);
                    draw_text(
                        text2,
                        screen_width() * 3. / 5. - text_size.width / 2.,
                        screen_height() * 3. / 4. - text_size.height / 2. + 50.,
                        font_size,
                        WHITE,
                    );
                    let text2 = "Aperte [q] para voltar ao menu";
                    let text_size = measure_text(text, None, font_size as _, 1.0);
                    draw_text(
                        text2,
                        screen_width() * 3. / 5. - text_size.width / 2.,
                        screen_height() * 3. / 4. - text_size.height / 2. + 80.,
                        font_size,
                        WHITE,
                    );
                    if is_key_pressed(KeyCode::Q) {
                        return false;
                    }
                    if is_key_pressed(KeyCode::Escape) {
                        let mut timer_count = 3;

                        loop {
                            if timer_count == 0 {
                                break;
                            }

                            draw_screen(bg_texture, atletico_logo, &mut ship, &asteroids);

                            let text = &format!("Retomando em {} ...", timer_count);
                            let font_size = 40.;
                            let text_size = measure_text(text, None, font_size as _, 1.0);
                            draw_text(
                                text,
                                screen_width() / 2. - text_size.width / 2.,
                                screen_height() * 3. / 4. - text_size.height / 2.,
                                font_size,
                                WHITE,
                            );

                            next_frame().await;
                            
                            let old = macroquad::time::get_time();

                            loop {
                                let now = macroquad::time::get_time();
                                if now - old >= 1.0 {
                                    break;
                                }
                            }

                            timer_count -= 1;
                            
                        }

                        game_state = AsteroidsState::Running;
                        continue;
                    }

                    next_frame().await;
                    continue;
                }
    
                AsteroidsState::Win => {
                    // Desenha o fundo de vitoria
                    draw_texture_ex(
                        win_bg_texture,
                        0.0,
                        0.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width(), screen_height())),
                            ..Default::default()
                        },
                    );

                    let text = "Aperte [enter] para jogar novamente";
                    let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);

                    draw_text(
                        text,
                        screen_width() / 2. - text_size.width / 2.,
                        screen_height() / 3. - text_size.height / 2.,
                        TEXT_SIZE as f32,
                        WHITE,
                    );

                    let text = "Aperte [q] para voltar ao menu";
                    let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);

                    draw_text(
                        text,
                        screen_width() / 2. - text_size.width / 2.,
                        screen_height() / 3. + (35.) - text_size.height / 2.,
                        TEXT_SIZE as f32,
                        WHITE,
                    );

                    if is_key_down(KeyCode::Enter) {//Após ganhar o jogo, se apertar enter,reseta as variáveis
                        ship = Ship::new(cruzeiro_logo, bullet_texture);
                        asteroids = (0..ASTEROID_COUNT).map(|_| Asteroid::new()).collect();
                        collided = false;

                        game_state = AsteroidsState::Startup;
                    }

                    if is_key_down(KeyCode::Q) {
                        return false;
                    }
                    next_frame().await;
                    continue;
                }
    
                AsteroidsState::Lose => {
                    // Desenha o fundo de derrota
                    draw_texture_ex(
                        lose_bg_texture,
                        0.0,
                        0.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width(), screen_height())),
                            ..Default::default()
                        },
                    );

                    let text = "Aperte [enter] para jogar novamente";
                    let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);

                    draw_text(
                        text,
                        screen_width() / 2. - text_size.width / 2.,
                        screen_height() / 3. - text_size.height / 2.,
                        TEXT_SIZE as f32,
                        WHITE,
                    );

                    let text = "Aperte [q] para voltar ao menu";
                    let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);

                    draw_text(
                        text,
                        screen_width() / 2. - text_size.width / 2.,
                        screen_height() / 3. + (35.) - text_size.height / 2.,
                        TEXT_SIZE as f32,
                        WHITE,
                    );

                    if is_key_down(KeyCode::Enter) {//Após perder o jogo, se apertar enter,reseta as variáveis
                        ship = Ship::new(cruzeiro_logo, bullet_texture);
                        asteroids = (0..ASTEROID_COUNT).map(|_| Asteroid::new()).collect();
                        collided = false;

                        game_state = AsteroidsState::Startup;
                    }

                    if is_key_down(KeyCode::Q) {
                        return false;
                    }
                    next_frame().await;
                    continue;
                }
    
                _ => {}
    
            }

            // Pausa o jogo
            if is_key_pressed(KeyCode::Escape) {
                game_state = AsteroidsState::Paused;
                next_frame().await;
                continue;
            }            

            // Verifica as colisões entre bullets e asteroids para divisão em asteroids menores
            let mut indexes_to_remove = HashSet::new();
            let mut asteroids_to_add = vec![];
            let mut bullets_to_remove = vec![];
            for (i, asteroid) in asteroids.iter_mut().enumerate() {
                for (b, bullet) in ship.bullets.iter().enumerate() {
                    if asteroid.collided(&bullet.position) {
                        bullets_to_remove.push(b);
                        let a = asteroid.resize();
                        match a {
                            Some(asteroid) => asteroids_to_add.push(asteroid),
                            None => {
                                indexes_to_remove.insert(i);
                            }
                        }
                    }
                }

                // Verificação de colisão entre ship e algum asteroid
                if asteroid.collided(&ship.position) && !(ship.has_shield) {
                    collided = true;
                }
            }

            // Verificação para derrota
            if collided {
                game_state = AsteroidsState::Lose;
                continue;
                
            }

            // Remoção de elementos
            for (num_removed, i) in indexes_to_remove.into_iter().enumerate() {
                asteroids.remove(i - num_removed);
            }
            for (num_removed, i) in bullets_to_remove.into_iter().enumerate() {
                ship.bullets.remove(i - num_removed);
            }
            for i in asteroids_to_add {
                asteroids.push(i);
            }

            // Verificação de vitória
            if asteroids.is_empty() {
                game_state = AsteroidsState::Win;
                continue;
            }

            // Atualização de posição de elementos
            ship.mv();
            asteroids.iter_mut().for_each(|a| a.mv());

            next_frame().await;

        }

    }

}