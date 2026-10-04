import { getSettings } from './api';
import type { Setting } from '$lib/types';

export interface AppFeatures {
  businessType: string;
  expiry: boolean;
  dose: boolean;
  lots: boolean;
  suppliers: boolean;
}

class FeaturesManager {
  // Global reactive state with default values representing Pharmacy (backward compatible)
  state = $state<AppFeatures>({
    businessType: 'pharmacy',
    expiry: true,
    dose: true,
    lots: true,
    suppliers: true
  });

  /**
   * Load features settings from the database and update the reactive state.
   */
  async load() {
    try {
      const settings = await getSettings();
      const map = new Map(settings.map(s => [s.key, s.value]));

      // Update state fields reactively
      this.state.businessType = map.get('business_type') || 'pharmacy';
      this.state.expiry = map.get('feature_expiry') !== '0';
      this.state.dose = map.get('feature_dose') !== '0';
      this.state.lots = map.get('feature_lots') !== '0';
      this.state.suppliers = map.get('feature_suppliers') !== '0';
    } catch (e) {
      console.error('Error loading system features configuration:', e);
    }
  }
}

export const featuresManager = new FeaturesManager();
