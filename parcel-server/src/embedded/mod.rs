use std::{collections::HashMap, fmt::Display, marker::PhantomData, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub mod postgresql;
pub mod redis;

#[async_trait::async_trait]
pub trait SetupStep {
    type SetupState;
    type SetupError;
    type Software;

    fn step_name(&self) -> String;
    async fn execute(
        &mut self,
        software: &mut Self::Software,
        state: Self::SetupState,
        progress_bar: &mut ProgressBar,
    ) -> Result<Self::SetupState, Self::SetupError>;
}

#[async_trait::async_trait]
pub trait EmbeddedSoftware {
    type SetupState;
    type SetupError: Display;
    type RunValue;
    type RunError;
    type StopError;

    fn get_name(&self) -> String;
    fn is_installed(&self) -> bool;
    async fn setup(
        &mut self,
    ) -> (
        Self::SetupState,
        Vec<
            Box<
                dyn SetupStep<
                    SetupState = Self::SetupState,
                    SetupError = Self::SetupError,
                    Software = Self,
                >,
            >,
        >,
    );
    async fn start(&mut self) -> Result<Self::RunValue, Self::RunError>;
    async fn stop(&mut self) -> Result<(), Self::StopError>;
}

pub struct EmbeddedSoftwareInstaller<
    Software,
    SetupState,
    SetupError,
    RunValue,
    RunError,
    StopError,
> where
    Software: EmbeddedSoftware<
        SetupState = SetupState,
        SetupError = SetupError,
        RunValue = RunValue,
        RunError = RunError,
        StopError = StopError,
    >,
{
    software: Software,
    _phantom_setup_error: PhantomData<SetupError>,
    _phantom_setup_state: PhantomData<SetupState>,
    _phantom_run_value: PhantomData<RunValue>,
    _phantom_run_error: PhantomData<RunError>,
    _phantom_stop_error: PhantomData<StopError>,
}

impl<Software, SetupState, SetupError, RunValue, RunError, StopError>
    EmbeddedSoftwareInstaller<Software, SetupState, SetupError, RunValue, RunError, StopError>
where
    SetupError: Display,
    Software: EmbeddedSoftware<
        SetupState = SetupState,
        SetupError = SetupError,
        RunValue = RunValue,
        RunError = RunError,
        StopError = StopError,
    >,
{
    pub fn new(software: Software) -> Self {
        Self {
            software,
            _phantom_setup_error: PhantomData,
            _phantom_setup_state: PhantomData,
            _phantom_run_value: PhantomData,
            _phantom_run_error: PhantomData,
            _phantom_stop_error: PhantomData,
        }
    }

    pub async fn setup(&mut self) -> Result<(), SetupError> {
        let pb_style: ProgressStyle = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] {msg} [{bar:40.cyan/blue}] {pos}/{len}",
        )
        .unwrap()
        .progress_chars("##-");
        let pb_style_waiting: ProgressStyle =
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg} (waiting)")
                .unwrap()
                .progress_chars("##-");
        let pb_style_failed: ProgressStyle =
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
                .progress_chars("##-");
        const STEADY_TICK_INTERVAL: Duration = Duration::from_millis(100);

        let (initial_state, steps) = self.software.setup().await;

        if steps.is_empty() || self.software.is_installed() {
            return Ok(());
        }

        let multi_progress = MultiProgress::new();
        let total_progress = multi_progress.add(
            ProgressBar::new(steps.len() as u64)
                .with_message("Total progress")
                .with_style(pb_style.clone()),
        );

        total_progress.println(format!("Configuring {}", self.software.get_name()));
        total_progress.enable_steady_tick(STEADY_TICK_INTERVAL);

        let mut step_progress_bars = HashMap::with_capacity(steps.len());

        for (index, step) in steps.iter().enumerate() {
            let step_progress_bar = multi_progress.add(
                ProgressBar::new(100)
                    .with_message(step.step_name())
                    .with_style(pb_style_waiting.clone()),
            );

            step_progress_bar.tick();
            step_progress_bars.insert(index, step_progress_bar);
        }

        let mut state = initial_state;
        let mut error = None;
        for (index, mut step) in steps.into_iter().enumerate() {
            let progress_bar = step_progress_bars.get_mut(&index).unwrap();

            progress_bar.reset();
            progress_bar.enable_steady_tick(STEADY_TICK_INTERVAL);
            progress_bar.set_style(pb_style.clone());

            match step.execute(&mut self.software, state, progress_bar).await {
                Ok(new_state) => state = new_state,
                Err(err) => {
                    progress_bar.set_style(pb_style_failed.clone());
                    progress_bar.finish_with_message(format!(
                        "{} (failed: {})",
                        progress_bar.message(),
                        err
                    ));

                    error = Some(err);
                    break;
                }
            }

            total_progress.inc(1);
        }

        if let Some(error) = error {
            total_progress.set_style(pb_style_failed.clone());
            total_progress.println("Failed");
            total_progress.tick();
            Err(error)
        } else {
            total_progress.finish();
            total_progress.println(format!("Finished"));
            Ok(())
        }
    }

    #[inline]
    pub async fn start(&mut self) -> Result<RunValue, RunError> {
        self.software.start().await
    }

    #[inline]
    pub async fn stop(&mut self) -> Result<(), StopError> {
        self.software.stop().await
    }
}
