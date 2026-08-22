use crate::message::Message;
use crate::state::AppState;
use crate::theme::{escala_fonte, obter_cores};

use crate::message::{
    DesempenhoApp,
    FonteTamanhoApp,
    IdiomaApp,
    TemaApp,
};

use iced::{Alignment, Background, Border, Element, Length, Padding, Shadow};

use iced::widget::{
    button,
    column,
    container,
    pick_list,
    row,
    text,
    text_input,
    Space,
};

pub fn view(state: &AppState) -> Element<'_, Message> {

    // ============================================================
    // TEMA
    // ============================================================

    let cores = obter_cores(state.tema_atual);

    // ============================================================
    // TAMANHO DA FONTE
    // ============================================================

    let escala = escala_fonte(state.fonte_tamanho_atual);


    // ============================================================
    // CABEÇALHO
    // ============================================================

    let title_section = column![

        text("Configurações e Preferências")
            .size(24.0 * escala)
            .color(cores.texto),

        text("Gerencie sua conta, aparência e desempenho do aplicativo.")
            .size(13.0 * escala)
            .color(cores.texto_secundario),

    ]
    .spacing(2);


    let header = row![
        title_section,
    ]
    .align_y(Alignment::Center);


    // ============================================================
    // PERFIL
    // ============================================================

    let profile_title = text("Criar Perfil de Usuário")
        .size(15.0 * escala)
        .color(cores.texto);


    let input_nome = text_input(
        "Nome completo...",
        &state.profile_name,
    )
    .on_input(Message::ProfileNameChanged)
    .padding(10);


    let input_email = text_input(
        "E-mail (ex: user@vaultnote.com)...",
        &state.profile_email,
    )
    .on_input(Message::ProfileEmailChanged)
    .padding(10);


    let input_senha = text_input(
        "Senha de acesso...",
        &state.profile_password,
    )
    .on_input(Message::ProfilePasswordChanged)
    .secure(true)
    .padding(10);


    let btn_criar_perfil = button(
        text("Criar Perfil")
            .size(13.0 * escala)
            .color(cores.texto),
    )
    .padding(10);


    let profile_card = container(

        column![

            text("Insira seus dados para registrar um novo perfil local:")
                .size(12.0 * escala)
                .color(cores.texto_secundario),

            Space::new()
                .height(4),

            input_nome,

            input_email,

            input_senha,

            Space::new()
                .height(4),

            row![

                Space::new()
                    .width(Length::Fill),

                btn_criar_perfil,

            ],

        ]
        .spacing(8),

    )
    .padding(14)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card),
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        shadow: Shadow::default(),

        ..Default::default()
    });


    let profile_section = column![
        profile_title,
        profile_card,
    ]
    .spacing(5);


    // ============================================================
    // TAMANHO DA FONTE
    // ============================================================

    let fonte_title = text("Tamanho da Interface")
        .size(15.0 * escala)
        .color(cores.texto);


    let tamanhos_fonte = [
        FonteTamanhoApp::Pequeno,
        FonteTamanhoApp::Normal,
        FonteTamanhoApp::Grande,
        FonteTamanhoApp::MuitoGrande,
    ];


    let seletor_fonte = pick_list(
        tamanhos_fonte,
        Some(state.fonte_tamanho_atual),
        Message::MudarTamanhoFonte,
    )
    .placeholder("Selecione o tamanho...");


    let fonte_card = container(

        column![

            text("Tamanho das letras:")
                .size(12.0 * escala)
                .color(cores.texto_secundario),

            seletor_fonte,

            text("Aumente o tamanho da interface para facilitar a leitura.")
                .size(11.0 * escala)
                .color(cores.texto_secundario),

        ]
        .spacing(6),

    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card),
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        shadow: Shadow::default(),

        ..Default::default()
    });


    let fonte_section = column![
        fonte_title,
        fonte_card,
    ]
    .spacing(5);


    // ============================================================
    // APARÊNCIA
    // ============================================================

    let appearance_title = text("Aparência e Acessibilidade")
        .size(15.0 * escala)
        .color(cores.texto);


    let temas_disponiveis = [
        TemaApp::White,
        TemaApp::Dark,
    ];


    let seletor_tema = pick_list(
        temas_disponiveis,
        Some(state.tema_atual),
        Message::MudarTema,
    )
    .placeholder("Selecione o tema...");


    let appearance_card = container(

        column![

            text("Tema do Aplicativo:")
                .size(12.0 * escala)
                .color(cores.texto_secundario),

            seletor_tema,

        ]
        .spacing(6),

    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card),
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        shadow: Shadow::default(),

        ..Default::default()
    });


    let appearance_section = column![
        appearance_title,
        appearance_card,
    ]
    .spacing(5);


    // ============================================================
    // DESEMPENHO
    // ============================================================

    let performance_title = text("Desempenho e Economia")
        .size(15.0 * escala)
        .color(cores.texto);


    let niveis = [
        DesempenhoApp::SemAnimacoes,
        DesempenhoApp::Baixo,
        DesempenhoApp::Medio,
        DesempenhoApp::Alto,
    ];


    let seletor_desempenho = pick_list(
        niveis,
        Some(state.desempenho_atual),
        Message::MudarDesempenho,
    )
    .placeholder("Selecione o desempenho...");


    let performance_card = container(

        column![

            text("Perfil de efeitos visuais e desempenho:")
                .size(12.0 * escala)
                .color(cores.texto_secundario),

            seletor_desempenho,

        ]
        .spacing(6),

    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card),
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        shadow: Shadow::default(),

        ..Default::default()
    });


    let performance_section = column![
        performance_title,
        performance_card,
    ]
    .spacing(5);


    // ============================================================
    // IDIOMA
    // ============================================================

    let language_title = text("Idioma e Localidade")
        .size(15.0 * escala)
        .color(cores.texto);


    let idiomas_disponiveis = [
        IdiomaApp::Portugues,
        IdiomaApp::Ingles,
        IdiomaApp::Espanhol,
    ];


    let seletor_idioma = pick_list(
        idiomas_disponiveis,
        Some(state.idioma_atual),
        Message::MudarIdioma,
    )
    .placeholder("Selecione o idioma...");


    let language_card = container(

        column![

            text("Idioma da Interface:")
                .size(12.0 * escala)
                .color(cores.texto_secundario),

            seletor_idioma,

        ]
        .spacing(6),

    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card),
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        shadow: Shadow::default(),

        ..Default::default()
    });


    let language_section = column![
        language_title,
        language_card,
    ]
    .spacing(5);


    // ============================================================
    // SUPORTE
    // ============================================================

    let support_title = text("Suporte e Sobre")
        .size(15.0 * escala)
        .color(cores.texto);


    let support_card = container(

        column![

            text("Central de Ajuda e FAQ")
                .size(12.0 * escala)
                .color(cores.texto),

            text("Versão do Aplicativo: v1.2.0 (Build 2026)")
                .size(11.0 * escala)
                .color(cores.texto_secundario),

        ]
        .spacing(2),

    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card),
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        shadow: Shadow::default(),

        ..Default::default()
    });


    let support_section = column![
        support_title,
        support_card,
    ]
    .spacing(5);


    // ============================================================
    // PÁGINA PRINCIPAL
    // ============================================================

    container(

        column![

            header,

            Space::new()
                .height(8),

            profile_section,

            fonte_section,

            appearance_section,

            performance_section,

            language_section,

            support_section,

        ]
        .spacing(12)
        .padding(Padding::new(24.0)),

    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.fundo),
        ),

        ..Default::default()
    })

    .into()
}